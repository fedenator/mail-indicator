pub mod mail;
pub mod config;
pub mod indicador;

use std::io::{ BufReader, BufRead, Write };
use std::sync::{ Arc, Mutex };

use crate::config::{ Config };
use crate::indicador::{ Indicador };
use crate::mail::{ Mail };

#[inline]
fn leer_mensaje_http(stream: &std::net::TcpStream) -> Result<String, ()> {
	let mut mensaje = String::new();

	let mut reader = BufReader::new(stream);

	let mut linea = String::new();

	loop {
		reader.read_line(&mut linea).map_err( |_| { () } )?;
		mensaje += &linea;

		if linea.trim().is_empty() {
			break;
		}
		linea.clear();
	}

	return Ok(mensaje);
}

#[inline]
fn responder_http_ok(stream: &mut std::net::TcpStream) {
	let mensaje = ":D";
	let respuesta = format!(
		"HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
		mensaje.len(),
		mensaje
	);

	stream.write_all( respuesta.as_bytes() ).unwrap();
}

#[inline]
fn recibir_redirect() -> Result<String, ()> {
	let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
	let stream = listener.incoming().next().unwrap();

	println!("Conexion recibida!");

	if let Ok(mut stream) = stream {
		let mensaje = leer_mensaje_http(&stream)?;
		responder_http_ok(&mut stream);

		println!("{}", mensaje);

		return Ok(mensaje);
	}

	return Err(());
}

#[inline]
fn abrir_link_en_navegador(link: &str) {
	std::process::Command::new("x-www-browser")
		.arg(link)
		.spawn()
		.expect("Error al abrir link en el navegador");
}

#[inline]
fn conseguir_gmail_oauth2_access_token() -> String {
	let config = oauth2::Config::new(
		"933010578097-4hvs3d2rcksvkdhq11nus75kn2kio4om.apps.googleusercontent.com",
		"3Y8HY6RlecfB0p_zZ8TReHcC",
		"https://accounts.google.com/o/oauth2/v2/auth",
		"https://www.googleapis.com/oauth2/v3/token",
	)
	.set_redirect_url("http://localhost:8080")

	//Añade el scope read-only de gmail
	.add_scope("https://www.googleapis.com/auth/gmail.readonly");

	abrir_link_en_navegador( &config.authorize_url().into_string() );
	recibir_redirect().unwrap();

	panic!("ASDASDSA");
	// return token_result.access_token;
}


fn main() {
	let config = Arc::new( Mutex::new( Config::new( conseguir_gmail_oauth2_access_token() ) ) );
	let config_clone_main = config.clone();

	gtk::init().expect("Error al iniciar GTK");

	let indicador = Arc::new(
		Mutex::new( Indicador::new( &config_clone_main.lock().unwrap() )
	));

	let mail = Arc::new( Mutex::new(
		Mail::new( config_clone_main.lock().unwrap().imap_config.clone() )
	));

	std::thread::Builder::new()
		.name( String::from("update-mail-indicator-thread") )
		.spawn(move || {
			loop {
				let mail_clone      = mail.clone();
				let indicador_clone = indicador.clone();
				mail_clone.lock().unwrap().actualizar();

				let config_clone_update_thread = config.clone();

				glib::source::idle_add( move || {
					indicador_clone.lock().unwrap().cambiar_icono(
						&config_clone_update_thread.lock().unwrap(),
						mail_clone.lock().unwrap().cantidad_mails_sin_leer.clone()
					);

					return glib::source::Continue(false);
				});

				std::thread::sleep( std::time::Duration::from_secs(15) );
			}
		}).expect("Fallo al crear update-mail-indicator-thread");

	gtk::main();
}
