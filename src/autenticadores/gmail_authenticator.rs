use oauth2::{ TokenResponse };

pub struct GMailOAuth2 {
	pub usuario     : String,
	pub access_token: crate::oauth::TokenResponse,
}

impl GMailOAuth2 {
	pub fn pedir_al_usuario() -> Self {
		return GMailOAuth2 {
			usuario     : String::from("fpalacios@scanntech.com"),
			access_token: conseguir_gmail_oauth2_access_token().unwrap(),
		};
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
		&self
	) -> imap::Session< native_tls::TlsStream<std::net::TcpStream> >
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
		).expect("Error al establecer la conexión con el servidor de correo.");

		return cliente_imap.authenticate("XOAUTH2", self)
			.expect("Error al autentificarse en el servidor de correo.");
	}
}

#[inline]
fn conseguir_gmail_oauth2_access_token() -> Result<crate::oauth::TokenResponse, ()> {
	let client = oauth2::basic::BasicClient::new(
		oauth2::ClientId::new( "933010578097-4hvs3d2rcksvkdhq11nus75kn2kio4om.apps.googleusercontent.com".into() ),
		Some( oauth2::ClientSecret::new( "3Y8HY6RlecfB0p_zZ8TReHcC".into() ) ),
		oauth2::AuthUrl::new( url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").map_err( |_| () )? ),
		Some( oauth2::TokenUrl::new( url::Url::parse("https://www.googleapis.com/oauth2/v3/token").map_err( |_| () )? ) ),
	);

	return crate::oauth::conseguir_oauth_access_token(
		client, vec![oauth2::Scope::new( "https://mail.google.com".into() )]
	);
}
