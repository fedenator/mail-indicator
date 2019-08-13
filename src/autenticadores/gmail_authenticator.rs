use oauth2::{ TokenResponse };

pub struct GMailOAuth2 {
	pub usuario     : String,
	pub cliente     : oauth2::basic::BasicClient,
	pub access_token: crate::oauth::StandardTokenResponse,
}

impl GMailOAuth2 {
	pub fn pedir_al_usuario() -> Result<Self, crate::oauth::ConseguirAccessTokenError> {
		let mut cliente = oauth2::basic::BasicClient::new(
			oauth2::ClientId::new( "933010578097-4hvs3d2rcksvkdhq11nus75kn2kio4om.apps.googleusercontent.com".into() ),
			Some( oauth2::ClientSecret::new( "3Y8HY6RlecfB0p_zZ8TReHcC".into() ) ),
			oauth2::AuthUrl::new( url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap() ),
			Some( oauth2::TokenUrl::new( url::Url::parse("https://www.googleapis.com/oauth2/v3/token").unwrap() ) ),
		);

		let access_token = conseguir_gmail_oauth2_access_token(&mut cliente)?;

		return Ok(GMailOAuth2 {
			usuario     : String::from("fedenator7@gmail.com"),
			cliente     : cliente,
			access_token: access_token,
		});
	}

	fn refreshear_token(
		&mut self
	) -> Result<(), crate::oauth::RefreshTokenError>
	{
		let nuevo_token = crate::oauth::refresh_token(&self.access_token, &mut self.cliente)?;

		self.access_token = nuevo_token;

		return Ok(());
	}
}

impl imap::Authenticator for GMailOAuth2 {
	type Response = String;

	fn process(&self, _data: &[u8]) -> Self::Response {
		return format!(
			"user={usuario}\x01auth=Bearer {access_token}\x01\x01",
			usuario      = self.usuario,
			access_token = self.access_token.access_token().secret(),
		);
	}
}

impl crate::autenticadores::ImapAutenticador for GMailOAuth2 {
	fn abrir_sesion(
		&mut self
	) -> Result<imap::Session< native_tls::TlsStream<std::net::TcpStream> >, crate::autenticadores::AbrirSesionError>
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

		match cliente_imap.authenticate("XOAUTH2", self) {
			Ok(sesion) => {
				return Ok(sesion);
			}
			// En caso de que falle al autentificarse intenta refreshear el token
			// ya que no hay forma de saber porque falló.
			Err( (_error, cliente_imap) ) => {
				self.refreshear_token()
					.map_err(Box::new)
					.map_err( |e| crate::autenticadores::AbrirSesionError::AlAutentificarse{causa: e} )?;

				return cliente_imap.authenticate("XOAUTH2", self)
					.map_err( |(e, _cliente)| e)
					.map_err(Box::new)
					.map_err( |e| crate::autenticadores::AbrirSesionError::AlAutentificarse{causa: e} );
			}
		}
	}
}

#[inline]
fn conseguir_gmail_oauth2_access_token(
	cliente: &mut oauth2::basic::BasicClient
) -> Result<crate::oauth::StandardTokenResponse, crate::oauth::ConseguirAccessTokenError>
{
	return crate::oauth::conseguir_access_token(
		cliente, vec![oauth2::Scope::new( "https://mail.google.com".into() )]
	);
}
