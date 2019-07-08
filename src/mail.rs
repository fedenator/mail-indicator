use std::sync::{ Arc };

use crate::config::{ ImapConfig };

/// Componente generico con las capacidades basicas de trabajar con emails
pub struct Mail {
	pub imap_config            : Arc<ImapConfig>,
	pub cantidad_mails_sin_leer: u32,
}

impl Mail {

	pub fn new(imap_config: Arc<ImapConfig>) -> Self {
		return Mail {
			imap_config            : imap_config,
			cantidad_mails_sin_leer: 0_u32,
		};
	}

	pub fn actualizar(&mut self) {
		let gmail_auth = GMailOAuth2 {
			usuario     : self.imap_config.username.clone(),
			access_token: self.imap_config.access_token.clone()
		};

		let conector_tls = native_tls::TlsConnector::builder()
			.build()
			.expect("Error al crear el conector tls");

		// NOTE(fpalacios): Todavia no entiendo porque hay que pasar el dominio 2 veces,
		// segun el autor de la libreria es para checkear que el certificado TLS sea valido
		// pero la verdad que podria clonarlo la libreria. Capaz que hay que leer mas sobre esto.
		let cliente_imap = imap::connect(
			( self.imap_config.dominio.clone(), self.imap_config.puerto.clone() ),
			self.imap_config.dominio.clone(),
			&conector_tls
		).expect("Error al establecer la conexión con el servidor de correo.");

		let mut sesion_imap = cliente_imap.authenticate("XOAUTH2", &gmail_auth)
			.expect("Error al autentificarse en el servidor de correo.");

		let unseen: imap::types::Mailbox = sesion_imap.select("UNSEEN")
			.expect("Error al traer el inbox");

		self.cantidad_mails_sin_leer = unseen.exists;
	}
}

/*------------ Implementaciones especificas de imap::Authenticator por proveedor --------------- */
//GMAIL
struct GMailOAuth2 {
	usuario     : String,
	access_token: String,
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
