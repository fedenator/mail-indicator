use oauth2::TokenResponse;

#[derive(Debug, serde::Deserialize)]
pub struct GoogleApiInfo {
	pub issuer                               : String,
	pub authorization_endpoint               : String,
	pub token_endpoint                       : String,
	pub userinfo_endpoint                    : String,
	pub revocation_endpoint                  : String,
	pub jwks_uri                             : String,
	pub response_types_supported             : Vec<String>,
	pub subject_types_supported              : Vec<String>,
	pub id_token_signing_alg_values_supported: Vec<String>,
	pub scopes_supported                     : Vec<String>,
	pub token_endpoint_auth_methods_supported: Vec<String>,
	pub claims_supported                     : Vec<String>,
	pub code_challenge_methods_supported     : Vec<String>,
}

#[derive(failure::Fail, Debug)]
pub enum ObtenerConsultandoError {
	#[fail(display = "Error al hacer peticion [{:?}]", causa)]
	AlHacerPeticion{ causa: reqwest::Error },

	#[fail(display = "Error al deserializar JSON [{:?}]", causa)]
	AlDeserializarJson{ causa: reqwest::Error },
}

impl GoogleApiInfo {
	pub fn obtener_consultando() -> Result<Self, ObtenerConsultandoError> {
		return reqwest::get("https://accounts.google.com/.well-known/openid-configuration")
			.map_err( |e| ObtenerConsultandoError::AlHacerPeticion{causa: e} )?
			.json()
			.map_err( |e| ObtenerConsultandoError::AlDeserializarJson{causa: e} );
	}
}

#[derive(serde::Deserialize)]
pub struct UserInfoEmail {
	pub sub           : String,
	pub picture       : String,
	pub email         : String,
	pub email_verified: bool,
}

impl UserInfoEmail {
	pub fn obtener_usando_oauth_google_api(
		api_info    : &GoogleApiInfo,
		access_token: &crate::oauth::StandardTokenResponse,
	) -> Result<UserInfoEmail, reqwest::Error>
	{
		return reqwest::Client::new()
			.get(&api_info.userinfo_endpoint)
			.bearer_auth( &access_token.access_token().secret() )
			.send()?
			.json()
	}
}

pub fn obtener_email(
	access_token: &crate::oauth::StandardTokenResponse,
) -> Result<String, reqwest::Error>
{
	let api_info = crate::google_api::GoogleApiInfo::obtener_consultando().unwrap();

	return Ok(
		UserInfoEmail::obtener_usando_oauth_google_api(&api_info, access_token)?.email
	);
}