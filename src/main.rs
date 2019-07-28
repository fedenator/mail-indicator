pub mod http;
pub mod mail;
pub mod oauth;
pub mod config;
pub mod indicador;
pub mod autenticadores;

use std::sync::{ Arc, Mutex };

use crate::mail::{ Mail };
use crate::config::{ Config };
use crate::indicador::{ Indicador };
use crate::autenticadores::gmail_authenticator::{ GMailOAuth2 };

fn main() {
	gtk::init().expect("Error al iniciar GTK");

	let config = Arc::new( Mutex::new( Config::new() ) );

	let indicador = Arc::new(
		Mutex::new( Indicador::new( &config.lock().unwrap() )
	));

	let mail = Mail::new( GMailOAuth2::pedir_al_usuario() );

	std::thread::Builder::new()
		.name( String::from("update-mail-indicator-thread") )
		.spawn(move || {
			loop {
				let indicador_clone  = indicador.clone();
				let count_mails_sin_leer = mail.count_mails_sin_leer();

				let config_clone_update_thread = config.clone();

				glib::source::idle_add( move || {
					indicador_clone.lock().unwrap().cambiar_icono(
						&config_clone_update_thread.lock().unwrap(),
						count_mails_sin_leer
					);

					return glib::source::Continue(false);
				});

				std::thread::sleep( std::time::Duration::from_secs(15) );
			}
		}).expect("Fallo al crear update-mail-indicator-thread");

	gtk::main();
}
