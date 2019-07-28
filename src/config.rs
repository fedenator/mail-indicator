use std::path::{ PathBuf };

/*--------------------------------- Wapper de todas las configs ---------------------------------*/
pub struct Config {
	pub assets_folder: PathBuf,
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}

impl Config {
	pub fn new() -> Self {
		return Config {
			assets_folder: std::env::current_dir().unwrap().join("assets"),
		};
	}
}


/*------------------------------------ Configuracion de IMAP ------------------------------------*/
pub struct ImapConfig {
	pub username    : String,
	pub access_token: String,
}

impl ImapConfig {
	pub fn new(access_token: String) -> Self {
		return ImapConfig {
			username    : String::from("fedenator7@gmail.com"),
			access_token: access_token,
		};
	}
}
