pub mod gmail;

#[derive(failure::Fail, Debug)]
pub enum AbrirSesionError {
	#[fail(display = "Error al autentificarse contra el servidor de autentificacion [{:?}]", causa)]
	AlAutentificarse{ causa: Box<failure::Fail> },

	#[fail(display = "Error al comunicarse con el servidor de autentificación[{:?}]", causa)]
	AlComunicarse{ causa: Box<failure::Fail> },
}

/// Trait que sabe como abrir una sesion imap
pub trait ImapAutenticador {
	fn abrir_sesion(
		&mut self
	) -> Result<imap::Session< native_tls::TlsStream<std::net::TcpStream> >, AbrirSesionError>;
}
