use std::convert::{ TryFrom };

/// Componente generico con las capacidades basicas de trabajar con emails
pub struct Mail<Autenticador>
where
	Autenticador: crate::autenticadores::ImapAutenticador
{
	//TODO(fpalacios): hacer generica la respuesta
	autenticador: Autenticador,
}

impl<Autenticador> Mail<Autenticador>
where
	Autenticador: crate::autenticadores::ImapAutenticador
{
	pub fn new(autenticador: Autenticador) -> Self {
		return Mail {
			autenticador: autenticador,
		};
	}

	// TODO(fpalacios): Cambiar para contar las conversaciones sin leer,
	// en vez de contar emails individuales sin leer
	pub fn count_mails_sin_leer(&mut self) -> u32 {
		// TODO(fpalacios): Manejar este error con más gracia
		let mut sesion_imap = self.autenticador.abrir_sesion()
			.expect("Error al abrir la sesion imap con el servidor.");

		sesion_imap.examine("INBOX").expect("Error al traer el inbox");

		let unseen = sesion_imap.search("(UNSEEN)").unwrap();
		let unseen_count = u32::try_from( unseen.len() ).unwrap();

		return unseen_count;
	}
}
