use std::path::{ PathBuf };
use std::sync::{ Arc };

/*--------------------------------- Wapper de todas las configs ---------------------------------*/
pub struct Config {
	pub assets_folder: PathBuf,
	pub imap_config  : Arc<ImapConfig>,
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}

impl Config {
	pub fn new(access_token: String) -> Self {
		return Config {
			assets_folder: std::env::current_dir().unwrap().join("assets"),
			imap_config  : Arc::new( ImapConfig::new(access_token) ),
		};
	}
}


/*------------------------------------ Configuracion de IMAP ------------------------------------*/
pub struct ImapConfig {
	pub username     : String,
	pub access_token : String,
	pub dominio      : &'static str,
	pub puerto       : u16,
}

impl ImapConfig {
	pub fn new(access_token: String) -> Self {
		return ImapConfig {
			username     : String::from("fpalacios@scanntech.com"),
			access_token : access_token,
			dominio      : "imap.gmail.com",
			puerto       : 993,
		};
	}
}
