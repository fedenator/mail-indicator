const PUERTO_REDIRECCION_OAUTH2: u16 = 12345_u16;

pub type TokenResponse = oauth2::StandardTokenResponse< oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType >;

/// Retorna el token de access_grant a partir de la peticion que envia el browser cuando
/// el usuario autoriza a la aplicacion, puede fallar en caso de que no exista el token
#[inline]
fn obtener_authorization_code_de_redirect_oauth(
	request   : http::Request<()>,
	csrf_token: &oauth2::CsrfToken,
) -> Result<oauth2::AuthorizationCode, ()>
{
	//HACK(fpalacios): Concatena http://localhost antes del url que mando el server
	//                 porque la libreria de parsear url no reconoce url relativas o parciales
	let url = url::Url::parse( &format!( "http://localhost{}", request.uri() ) )
		.map_err( |_| () )?;

	let mut code  : Option<String> = None;
	let mut status: Option<String> = None;

	for (key, value) in url.query_pairs() {
		match key.as_ref() {
			"code" => {
				code = Some( value.into() );
			}
			"status" => {
				status = Some( value.into() );
			}
			_ => {}
		};
	}

	//NOTE(fpalacios): No estoy seguro que 100% de las veces sea necesario tener el status.
	//                 tiene sentido que sea obligatorio cuando tenes un csrf_token para
	//                 checkearlo, pero quisas algun autenticador no pueda conseguirlo.
	//                 Habria que investigar más sobre eso.
	if let ( Some(code), Some(status) ) = (code, status) {
		if csrf_token.secret() == &status {
			println!("Token status valid");
			return Ok( oauth2::AuthorizationCode::new( code.to_owned() ) )
		}
	}

	return Err(());
}

/// Retorna el token de access_grant que envia el navegador mediante un redirect a la url de redireccion
#[inline]
fn atender_redirect(
	csrf_token: &oauth2::CsrfToken,
	puerto    : u16,
) -> Result<oauth2::AuthorizationCode, ()> {
	// TODO(fpalacios): Hacer algo cuando el puerto no este disponible.
	//                  Capaz probar con otros puertos?
	let listener = std::net::TcpListener::bind(
		format!("127.0.0.1:{}", puerto)
	).unwrap();
	let stream = listener.incoming().next().unwrap();

	if let Ok(mut stream) = stream {
		let mut result: Result<oauth2::AuthorizationCode, ()> = Err( () );

		let request = crate::http::leer_mensaje_http(&stream)?;

		if let Ok(access_grant) = obtener_authorization_code_de_redirect_oauth(request, &csrf_token) {
			result = Ok(access_grant);
		}

		//TODO(fpalacios): Poner un mensaje más copado
		crate::http::responder_http_ok(&mut stream, ":D");

		return result;
	}

	return Err(());
}

#[inline]
fn abrir_url_en_navegador(url: &url::Url) {
	std::process::Command::new("x-www-browser")
		.arg( url.as_str() )
		.spawn()
		.expect("Error al abrir link en el navegador");
}

/// Le pide al usuario que autorice en el navegador, atiende el redirect y retorna el token
/// si no hubo ningun problema
pub fn conseguir_oauth_access_token(
	mut client: oauth2::basic::BasicClient,
	scopes: Vec<oauth2::Scope>,
) -> Result<crate::oauth::TokenResponse, ()>
{
	client = client.set_redirect_url(
		oauth2::RedirectUrl::new(
			url::Url::parse( &format!("http://localhost{}", PUERTO_REDIRECCION_OAUTH2) )
			.map_err( |_| () )?
		)
	);

	let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

	let mut auth_url_builder = client
		.authorize_url(oauth2::CsrfToken::new_random);

	for scope in scopes {
		auth_url_builder = auth_url_builder.add_scope(scope);
	}

	let (auth_url, csrf_token) = auth_url_builder
		.set_pkce_challenge(pkce_challenge)
		.url();

	abrir_url_en_navegador(&auth_url);

	let access_grant = atender_redirect(&csrf_token, PUERTO_REDIRECCION_OAUTH2)?;

	let access_token = client.exchange_code(access_grant)
		.set_pkce_verifier(pkce_verifier)
		.request(&oauth2::reqwest::http_client)
		.map_err( |_| () )?;

	return Ok( access_token );
}
