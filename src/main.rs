pub mod http;
pub mod mail;
pub mod oauth;
pub mod config;
pub mod indicador;
pub mod autenticadores;

use std::sync::{ Arc, Mutex };

use crate::config::{ Config };
use crate::indicador::{ Indicador };
use crate::mail::{ Mail };

fn main() {
	let config = Arc::new( Mutex::new( Config::new(
		crate::autenticadores::gmail_authenticator::conseguir_gmail_oauth2_access_token()
			.expect("No se puedo conseguir el token de autentificaciond de gmail")
	)));

	println!("Access Token conseguido con exito. {}", config.lock().unwrap().imap_config.access_token);

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
