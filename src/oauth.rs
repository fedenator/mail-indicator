/// Retorna el token de access_grant a partir de la peticion que envia el browser cuando
/// el usuario autoriza a la aplicacion, puede fallar en caso de que no exista el token
#[inline]
fn obtener_access_grant_de_redirect_oauth(
	request: http::Request<()>
) -> Result<String, ()>
{
	let url = url::Url
		::parse( &format!( "http://localhost{}", request.uri() ) )
		.map_err( |_| () )?;


	for (key, value) in url.query_pairs() {
		if key == "code" {
			return Ok( value.to_string() );
		}
	}

	return Err(());
}

/// Retorna el token de access_grant que envia el navegador mediante un redirect a la url de redireccion
#[inline]
fn atender_redirect() -> Result<String, ()> {
	let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
	let stream = listener.incoming().next().unwrap();

	if let Ok(mut stream) = stream {
		let mut result: Result<String, ()> = Err( () );

		let request = crate::http::leer_mensaje_http(&stream)?;

		if let Ok(access_grant) = obtener_access_grant_de_redirect_oauth(request) {
			result = Ok(access_grant);
		}

		crate::http::responder_http_ok(&mut stream, ":D");

		return result;
	}

	return Err(());
}

#[inline]
fn abrir_link_en_navegador(link: &str) {
	std::process::Command::new("x-www-browser")
		.arg(link)
		.spawn()
		.expect("Error al abrir link en el navegador");
}

/// Le pide al usuario que autorice en el navegador, atiende el redirect y retorna el token
/// si no hubo ningun problema
pub fn conseguir_oauth_access_token(
	mut config: oauth2::Config
) -> Result<String, ()>
{
	config = config.set_redirect_url("http://localhost:8080");

	abrir_link_en_navegador( &config.authorize_url().into_string() );
	let access_grant = atender_redirect()?;

	let access_token = config.exchange_code(access_grant);

	return Ok(access_token.unwrap().access_token);
}
