use oauth2::{ TokenResponse };

pub type SesionImap  = imap::Session< native_tls::TlsStream<std::net::TcpStream> >;
type     ClienteImap = imap::Client< native_tls::TlsStream<std::net::TcpStream> >;

pub struct GMailOAuth2Autenticador {
	usuario      : String,
	cliente_oauth: oauth2::basic::BasicClient,
	access_token : crate::oauth::StandardTokenResponse,
}

impl GMailOAuth2Autenticador {
	pub fn pedir_al_usuario() -> Result<Self, crate::oauth::ConseguirAccessTokenError> {
		let mut cliente_oauth = crear_cliente_oauth2();
		let access_token = conseguir_gmail_oauth2_access_token(&mut cliente_oauth)?;

		let email = crate::google_api::obtener_email(&access_token)
			.map_err( |e| crate::oauth::ConseguirAccessTokenError::AlObtenerInfoDeCuenta{causa: e})?;

		return Ok(GMailOAuth2Autenticador {
			usuario      : email,
			cliente_oauth: cliente_oauth,
			access_token : access_token,
		});
	}

	#[inline]
	fn abrir_sesion_imap(
		&self,
		cliente_imap: ClienteImap
	) -> Result<SesionImap, (imap::error::Error, ClienteImap)>
	{
		return cliente_imap.authenticate("XOAUTH2", self);
	}

	/// Recarga el access_token pidiendole al usuario que se autentifique denuevo.
	fn recargar_access_token(
		&mut self,
		cliente_imap: ClienteImap
	) -> Result<SesionImap, crate::autenticadores::AbrirSesionError>
	{
		let access_token = conseguir_gmail_oauth2_access_token( &mut crear_cliente_oauth2() )
			.map_err( |error| {
				return crate::autenticadores::AbrirSesionError::AlComunicarse {
					causa: Box::new(error)
				};
			})?;

		self.access_token = access_token;
		return self.abrir_sesion_imap(cliente_imap)
			.map_err( |(error, _cliente)| {
				match error {
					imap::error::Error::Io{..}
					| imap::error::Error::ConnectionLost{..}
					| imap::error::Error::Tls{..}
					| imap::error::Error::TlsHandshake{..}
					| imap::error::Error::Append{..}
					| imap::error::Error::Bad{..}
					| imap::error::Error::Parse{..}
					| imap::error::Error::Validate{..}
					 => {
						return crate::autenticadores::AbrirSesionError::AlComunicarse {
							causa: Box::new(error)
						};
					}
					imap::error::Error::No {..} => {
						return crate::autenticadores::AbrirSesionError::AlAutentificarse {
							causa: Box::new(error)
						};
					}
				}
			});
	}

	fn refreshear_token(
		&mut self
	) -> Result<(), crate::oauth::RefreshTokenError>
	{
		let nuevo_token = crate::oauth::refresh_token(&self.access_token, &mut self.cliente_oauth)?;

		self.access_token = nuevo_token;

		return Ok(());
	}
}

impl imap::Authenticator for GMailOAuth2Autenticador {
	type Response = String;

	fn process(&self, _data: &[u8]) -> Self::Response {
		return format!(
			"user={usuario}\x01auth=Bearer {access_token}\x01\x01",
			usuario      = self.usuario,
			access_token = self.access_token.access_token().secret(),
		);
	}
}

impl crate::autenticadores::ImapAutenticador for GMailOAuth2Autenticador {


	//TODO(fpalacios): Hay mucho manejo de error hecho a mano en esta funcion
	//                 habria que encontrar una forma más elegante.

	/// Para abir la sesion se intenta:
	/// 1) Usar el ultimo access_token
	/// 2) Usar el refresh_token(si hay alguno) para refreshear el access_token
	/// 3) Pedir un access_token nuevo al usuario
	fn abrir_sesion(
		&mut self
	) -> Result<SesionImap, crate::autenticadores::AbrirSesionError>
	{
		let conector_tls = native_tls::TlsConnector::builder()
			.build()
			.expect("Error al crear el conector tls");

		// NOTE(fpalacios): Todavia no entiendo porque hay que pasar el dominio 2 veces,
		// segun el autor de la libreria es para checkear que el certificado TLS sea valido
		// pero la verdad que podria clonarlo la libreria. Capaz que hay que leer mas sobre esto.
		let cliente_imap = imap::connect(
			("imap.gmail.com", 993),
			"imap.gmail.com",
			&conector_tls
		)
		.map_err( Box::new )
		.map_err( |e| crate::autenticadores::AbrirSesionError::AlComunicarse{causa: e} )?;

		match self.abrir_sesion_imap(cliente_imap) {
			Ok(sesion) => {
				return Ok(sesion);
			}
			Err( (_error, cliente_imap) ) => {

				// Si hay exito al refreshear el token se intenta de nuevo
				// con el access_token en la estructura.
				if let Ok(()) = self.refreshear_token() {
					match self.abrir_sesion_imap(cliente_imap) {
						Ok(nueva_sesion) => {
							info!("GMailAutenticator: Se refresheo el token con exito");
							return Ok(nueva_sesion);
						}
						Err( (_error, cliente_imap) ) => {
							return self.recargar_access_token(cliente_imap);
						}
					}
				}
				// Sino directamente se consigue un access_token nuevo.
				else {
					return self.recargar_access_token(cliente_imap);
				}
			}
		}
	}
}

fn crear_cliente_oauth2() -> oauth2::basic::BasicClient {
	return oauth2::basic::BasicClient::new(
		oauth2::ClientId::new( "933010578097-4hvs3d2rcksvkdhq11nus75kn2kio4om.apps.googleusercontent.com".into() ),
		Some( oauth2::ClientSecret::new( "3Y8HY6RlecfB0p_zZ8TReHcC".into() ) ),
		oauth2::AuthUrl::new( url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap() ),
		Some( oauth2::TokenUrl::new( url::Url::parse("https://www.googleapis.com/oauth2/v3/token").unwrap() ) ),
	);
}

#[inline]
fn conseguir_gmail_oauth2_access_token(
	cliente_oauth: &mut oauth2::basic::BasicClient
) -> Result<crate::oauth::StandardTokenResponse, crate::oauth::ConseguirAccessTokenError>
{
	return crate::oauth::conseguir_access_token(
		cliente_oauth,
		vec![
			oauth2::Scope::new( "https://mail.google.com".into() ), // scope para imap
			oauth2::Scope::new( "https://www.googleapis.com/auth/userinfo.email".into() ), // scope para obtener email
		]
	);
}
