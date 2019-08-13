use std::convert::{ TryFrom };

#[derive(failure::Fail, Debug)]
pub enum MailError {
	#[fail(display = "Error de imap [{:?}]", causa)]
	ErrorImap{ causa: imap::error::Error },

	#[fail(display = "Error al abrir sesion [{:?}]", causa)]
	AlAbrirSesion{ causa: crate::autenticadores::AbrirSesionError },

	#[fail(display = "Erro")]
	AlInterpretarMensaje
}

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
	pub fn cant_mails_sin_leer(&mut self) -> Result<u32, MailError> {
		// TODO(fpalacios): Manejar este error con más gracia
		let mut sesion_imap = self.autenticador.abrir_sesion()
			.map_err( |e| MailError::AlAbrirSesion{ causa: e } )?;

		sesion_imap.examine("INBOX")
			.map_err( |e| MailError::ErrorImap{ causa: e } )?;

		let unseen = sesion_imap.search("(UNSEEN)")
			.map_err( |e| MailError::ErrorImap{ causa: e } )?;

		let unseen_count = u32::try_from( unseen.len() )
			.map_err( |_| MailError::AlInterpretarMensaje )?;

		return Ok(unseen_count);
	}
}
