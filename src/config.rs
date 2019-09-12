use std::path::{ PathBuf };

use crate::autenticadores::gmail::autenticador::{ GMailOAuth2Autenticador };

/*------------------------------------ Configuracion general ------------------------------------*/
pub struct Config {
	pub carpeta_instalacion  : PathBuf,
	pub carpeta_assets       : PathBuf,
	pub carpeta_configuracion: PathBuf,
	pub carpeta_logs         : PathBuf,
}

// Implementa los marcadores para enviar entre hilos
unsafe impl Send for Config {}
unsafe impl Sync for Config {}

impl Config {
	pub fn new() -> Self {
		let carpeta_instalacion   = PathBuf::from("/etc/mail-indicator");
		let carpeta_home          = dirs::home_dir().expect("No se pudo encontrar la carpeta home");
		let carpeta_configuracion = carpeta_home.join(".config/mail-indicator");

		return Config {
			carpeta_instalacion  : carpeta_instalacion.clone(),
			carpeta_assets       : carpeta_instalacion.join("assets"),
			carpeta_configuracion: carpeta_configuracion.clone(),
			carpeta_logs         : carpeta_configuracion.join("logs"),
		};
	}
}

// Modificar esta funcion para que retorne el autentificador correspondiete a tu proveedor
// de correo.
pub fn autenticador() -> Result<GMailOAuth2Autenticador, crate::oauth::ConseguirAccessTokenError> {
	return GMailOAuth2Autenticador::pedir_al_usuario();
}
