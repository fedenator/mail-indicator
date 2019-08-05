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

	let config    = Config::new();
	let indicador = Indicador::new(&config);
	let mut mail  = Mail::new( GMailOAuth2::pedir_al_usuario() );

	std::thread::Builder::new()
		.name( String::from("update-mail-indicator-thread") )
		.spawn(move || {
			let config_arc    = Arc::new( config );
			let indicador_arc = Arc::new( Mutex::new(indicador) );

			loop {
				let config_clone    = config_arc.clone();
				let indicador_clone = indicador_arc.clone();

				let count_mails_sin_leer = mail.count_mails_sin_leer();

				glib::source::idle_add( move || {
					indicador_clone.lock().unwrap().cambiar_icono(
						&config_clone,
						count_mails_sin_leer
					);

					return glib::source::Continue(false);
				});

				std::thread::sleep( std::time::Duration::from_secs(15) );
			}
		}).expect("Fallo al crear update-mail-indicator-thread");

	gtk::main();
}
