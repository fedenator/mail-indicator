pub mod mail;
pub mod config;
pub mod indicador;

use std::sync::{ Arc, Mutex };

use crate::config::{ Config };
use crate::indicador::{ Indicador };
use crate::mail::{ Mail };

fn main() {
	let config = Arc::new( Config::new() );

	gtk::init().expect("Error al iniciar GTK");

	let mut indicador = Indicador::new( &config.clone() );
	let mut mail      = Mail::new( config.imap_config.clone() );

	let mail_arc      = Arc::new(mail);
	let indicador_arc = Arc::new( Mutex::new( indicador ) );

	std::thread::spawn(move || {
		loop {
			let mail_clone      = mail_arc.clone();
			let indicador_clone = indicador_arc.clone();
			mail_clone.actualizar();

			glib::source::idle_add( move || {
				indicador_clone.lock().unwrap().cambiar_icono(
					&config.clone(),
					mail_clone.cantidad_mails_sin_leer.clone()
				);

				return glib::source::Continue(false);
			});
		}
	});



	gtk::main();
}
