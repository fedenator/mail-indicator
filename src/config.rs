use std::path::{ PathBuf };
use std::sync::{ Arc };

/*--------------------------------- Wapper de todas las configs ---------------------------------*/
pub struct Config {
	pub assets_folder: PathBuf,
	pub imap_config  : Arc<ImapConfig>,
}

impl Config {
	pub fn new() -> Self {
		return Config {
			assets_folder: std::env::current_dir().unwrap().join("assets"),
			imap_config  : Arc::new( ImapConfig::new() ),
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
	pub fn new() -> Self {
		return ImapConfig {
			username     : String::from("fpalacios@scanntech.com"),
			access_token : String::from(""),
			dominio      : "imap.gmail.com",
			puerto       : 993,
		};
	}
}
