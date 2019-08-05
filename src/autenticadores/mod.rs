pub mod gmail_authenticator;

/// Trait que sabe como abrir una sesion imap
pub trait ImapAutenticador {

	fn abrir_sesion(
		&mut self
	) -> Result<imap::Session< native_tls::TlsStream<std::net::TcpStream> >, ()>;
}
