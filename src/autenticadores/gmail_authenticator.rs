pub fn conseguir_gmail_oauth2_access_token() -> Result<String, ()> {
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


//GMAIL
#[derive(Debug)]
pub struct GMailOAuth2 {
	pub usuario     : String,
	pub access_token: String,
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
