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
	pub fn count_mails_sin_leer(&mut self) -> Result<u32, ()> {
		// TODO(fpalacios): Manejar este error con más gracia
		let mut sesion_imap = self.autenticador.abrir_sesion()?;

		sesion_imap.examine("INBOX")
			.map_err( |_| () )?;

		let unseen = sesion_imap.search("(UNSEEN)")
			.map_err( |_| () )?;

		let unseen_count = u32::try_from( unseen.len() )
			.map_err( |_| () )?;

		return Ok(unseen_count);
	}
}
