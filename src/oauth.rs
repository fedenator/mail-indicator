use oauth2::{ TokenResponse };

const PUERTO_REDIRECCION_OAUTH2: u16 = 12345_u16;

pub type StandardTokenResponse = oauth2::StandardTokenResponse< oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType >;
pub type StandardErrorResponse = oauth2::RequestTokenError< oauth2::reqwest::Error, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType> >;

/// Retorna el token de access_grant a partir de la peticion que envia el browser cuando
/// el usuario autoriza a la aplicacion, puede fallar en caso de que no exista el token
#[inline]
fn obtener_authorization_code_de_redirect_oauth(
	request   : http::Request<()>,
	csrf_token: &oauth2::CsrfToken,
) -> Result<oauth2::AuthorizationCode, ()>
{
	// HACK(fpalacios): Concatena http://localhost antes del url que mando el server
	// porque la libreria de parsear url no reconoce url relativas o parciales
	let url = url::Url::parse( &format!( "http://localhost{}", request.uri() ) )
		.map_err( |_| () )?;

	let mut code  : Option<String> = None;
	let mut status: Option<String> = None;

	for (key, value) in url.query_pairs() {
		match key.as_ref() {
			"code" => {
				code = Some( value.into() );
			}
			"state" => {
				status = Some( value.into() );
			}
			_ => {}
		};
	}

	// NOTE(fpalacios): No estoy seguro que 100% de las veces sea necesario tener el status.
	// Tiene sentido que sea obligatorio cuando tenes un csrf_token para checkearlo,
	// pero quisas algun autenticador no pueda conseguirlo.
	// Habria que investigar más sobre eso.
	if let ( Some(code), Some(status) ) = (code, status) {
		if csrf_token.secret() == &status {
			println!("Token status valid");
			return Ok( oauth2::AuthorizationCode::new( code.to_owned() ) )
		}
	}

	return Err(());
}

#[derive(failure::Fail, Debug)]
pub enum AtenderRedirectError {
	#[fail(display = "Error al intentar abrir el puerto [{:?}]", causa)]
	AlAbrirPuerto{ causa: std::io::Error },

	#[fail(display = "Error al hablar con el cliente [{:?}]", causa)]
	AlHablarConCliente{ causa: std::io::Error },

	#[fail(display = "Error al leer el mensaje del cliente")]
	AlLeerMensaje,

	#[fail(display = "El servidor rechazó el token de refresh")]
	RefreshRechazado,
}

/// Retorna el token de access_grant que envia el navegador mediante un redirect a la url de redireccion
#[inline]
fn atender_redirect(
	csrf_token: &oauth2::CsrfToken,
	puerto    : u16,
) -> Result<oauth2::AuthorizationCode, AtenderRedirectError>
{
	// TODO(fpalacios): Hacer algo cuando el puerto no este disponible.
	// Capaz probar con otros puertos?
	let listener = std::net::TcpListener::bind(
		format!("127.0.0.1:{}", puerto)
	).map_err( |e| AtenderRedirectError::AlAbrirPuerto{causa: e} )?;

	//TODO(fpalacios): Revisar si listener.incoming() puede dar None en algun caso
	//                 Y seguro poner un log
	let mut stream = listener.incoming().next().unwrap()
		.map_err( |e| AtenderRedirectError::AlHablarConCliente{causa: e} )?;

	let request = crate::http::leer_mensaje_http(&stream)
		.map_err( |_| AtenderRedirectError::AlLeerMensaje )?;

	match obtener_authorization_code_de_redirect_oauth(request, &csrf_token) {
		Ok(access_grant) => {
			//TODO(fpalacios): Poner un mensaje más copado
			crate::http::responder_http_ok(&mut stream, ":D");

			return Ok(access_grant);
		}
		Err(_e) => {
			return Err(AtenderRedirectError::RefreshRechazado);
		}
	};

}

#[inline]
fn abrir_url_en_navegador(url: &url::Url) {
	std::process::Command::new("x-www-browser")
		.arg( url.as_str() )
		.spawn()
		.expect("Error al abrir link en el navegador");
}

#[derive(failure::Fail, Debug)]
pub enum ConseguirAccessTokenError {
	#[fail(display = "Error al interpretar la url")]
	AlInterpretarUrl,

	#[fail(display = "Error al atender el redirect del cliente [{:?}]", causa)]
	AlAtenderRedirect{ causa: AtenderRedirectError },

	#[fail(display = "Error al pedir al servidor [{:?}]", causa)]
	AlPedirAlServidor{ causa: StandardErrorResponse },
}

/// Le pide al usuario que autorice en el navegador, atiende el redirect y retorna el token
/// si no hubo ningun problema
pub fn conseguir_access_token(
	cliente: &mut oauth2::basic::BasicClient,
	scopes : Vec<oauth2::Scope>,
) -> Result<crate::oauth::StandardTokenResponse, ConseguirAccessTokenError>
{
	// TODO(fpalcios): Buscar una forma de no tener que clonar el cliente.
	// Quizas pedir owner de el cliente viejo y retornar el cliente modificado?
	let mut cliente = cliente.clone();
	cliente = cliente.set_redirect_url(
		oauth2::RedirectUrl::new(
			url::Url::parse( &format!("http://localhost:{}", PUERTO_REDIRECCION_OAUTH2) )
				.map_err( |_e| ConseguirAccessTokenError::AlInterpretarUrl )?
		)
	);

	let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

	let mut auth_url_builder = cliente.authorize_url(oauth2::CsrfToken::new_random);

	for scope in scopes {
		auth_url_builder = auth_url_builder.add_scope(scope);
	}

	let (auth_url, csrf_token) = auth_url_builder
		.set_pkce_challenge(pkce_challenge)
		.url();

	abrir_url_en_navegador(&auth_url);

	let access_grant = atender_redirect(&csrf_token, PUERTO_REDIRECCION_OAUTH2)
		.map_err( |e| ConseguirAccessTokenError::AlAtenderRedirect{ causa: e } )?;

	let access_token = cliente.exchange_code(access_grant)
		.set_pkce_verifier(pkce_verifier)
		.request(&oauth2::reqwest::http_client)
		.map_err( |e| ConseguirAccessTokenError::AlPedirAlServidor{ causa: e } )?;

	return Ok( access_token );
}

/// Errores que pueden surgir al intentar refreshear un access_token
#[derive(failure::Fail, Debug)]
pub enum RefreshTokenError {
	#[fail(display = "Error durante la peticion al servidor {:?}", causa)]
	AlHacerRequest{ causa: StandardErrorResponse },

	#[fail(display = "No hay token de refresh para hacer la peticion")]
	FaltaRefreshToken,
}

pub fn refresh_token(
	token_expirado: &crate::oauth::StandardTokenResponse,
	cliente       : &mut oauth2::basic::BasicClient,
) -> Result< StandardTokenResponse, RefreshTokenError>
{
	if let Some(refresh_token) = token_expirado.refresh_token() {
		return cliente.exchange_refresh_token(refresh_token)
			.request(&oauth2::reqwest::http_client)
			.map_err( |e| RefreshTokenError::AlHacerRequest{causa: e} );
	} else {
		return Err(RefreshTokenError::FaltaRefreshToken);
	}
}
