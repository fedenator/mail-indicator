pub struct GMailOAuth2 {
	pub usuario     : String,
	pub access_token: String,
}

impl GMailOAuth2 {
	pub fn pedir_al_usuario() -> Self {
		return GMailOAuth2 {
			usuario     : String::from("fedenator7@gmail.com"),
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
			access_token = self.access_token,
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
			( "imap.gmail.com", 993 ),
			"imap.gmail.com",
			&conector_tls
		).expect("Error al establecer la conexión con el servidor de correo.");

		return cliente_imap.authenticate("XOAUTH2", self)
			.expect("Error al autentificarse en el servidor de correo.");
	}
}

#[inline]
fn conseguir_gmail_oauth2_access_token() -> Result<String, ()> {
	let config = oauth2::Config::new(
		"933010578097-4hvs3d2rcksvkdhq11nus75kn2kio4om.apps.googleusercontent.com",
		"3Y8HY6RlecfB0p_zZ8TReHcC",
		"https://accounts.google.com/o/oauth2/v2/auth",
		"https://www.googleapis.com/oauth2/v3/token",
	)
	//Añade el scope read-only de gmail
	.add_scope("https://mail.google.com");

	return crate::oauth::conseguir_oauth_access_token(config);
}
