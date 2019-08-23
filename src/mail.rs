use std::convert::{ TryFrom };

#[derive(failure::Fail, Debug)]
pub enum MailError {
	#[fail(display = "Error de imap [{:?}]", causa)]
	ErrorImap{ causa: imap::error::Error },

	#[fail(display = "Error al abrir sesion [{:?}]", causa)]
	AlAbrirSesion{ causa: crate::autenticadores::AbrirSesionError },

	#[fail(display = "Error al interpretar el mensaje")]
	AlInterpretarMensaje
}

/// Componente generico con las capacidades basicas de trabajar con emails
pub struct Mail<Autenticador>
where
	Autenticador: crate::autenticadores::ImapAutenticador
{
	//TODO(fpalacios): hacer generica la respuesta
	pub autenticador: Autenticador,
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
		info!("Consultando cantidad de mails sin leer...");

		// TODO(fpalacios): Manejar este error con más gracia
		debug!("Abriendo sesion imap...");
		let mut sesion_imap = self.autenticador.abrir_sesion()
			.map_err( |e| MailError::AlAbrirSesion{ causa: e } )?;
		debug!("Sesion imap abierta.");

		debug!("Examinando INBOX...");
		sesion_imap.examine("INBOX")
			.map_err( |e| MailError::ErrorImap{ causa: e } )?;
		debug!("INBOX examinada.");

		debug!("Buscando (UNSEEN)'s...");
		let unseen = sesion_imap.search("(UNSEEN)")
			.map_err( |e| MailError::ErrorImap{ causa: e } )?;
		debug!("(UNSEEN)'s encontrados.");

		let unseen_count = u32::try_from( unseen.len() )
			.map_err( |_| MailError::AlInterpretarMensaje )?;

		info!("Se encontraron {} emails sin leer.", unseen_count);

		return Ok(unseen_count);
	}
}
