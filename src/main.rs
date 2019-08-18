#[macro_use]
extern crate log;

pub mod http;
pub mod mail;
pub mod oauth;
pub mod config;
pub mod indicador;
pub mod autenticadores;

use std::sync::{ Arc, Mutex };

use crate::mail::{ Mail };
use crate::indicador::{ Indicador };

fn iniciar_log4rs(config: &crate::config::Config) {
	let appender_consola = log4rs::append::console::ConsoleAppender::builder().build();

	let appender_archivo = log4rs::append::file::FileAppender::builder()
		.encoder( Box::new( log4rs::encode::pattern::PatternEncoder::new("{d} - {m}{n}") ) )
		.build( config.carpeta_logs.join("log.log") )
		.unwrap();

	let config = log4rs::config::Config::builder()
		// Apender que muestra en la consola
		.appender(
			log4rs::config::Appender::builder()
				.build("appender_consola", Box::new(appender_consola) )
		)
		// Appender que guarda en el log permanente
		.appender(
			log4rs::config::Appender::builder()
				.build("appender_archivo", Box::new(appender_archivo) )
		)
		.build(
			log4rs::config::Root::builder()
				.appender("appender_consola")
				.appender("appender_archivo")
				.build(log::LevelFilter::Warn)
		)
		.unwrap();

	log4rs::init_config(config)
		.expect("Error al iniciar la configuracion de log4rs");
}

fn main() {
	gtk::init().expect("Error al iniciar GTK");

	let config = crate::config::Config::new();

	iniciar_log4rs(&config);

	let indicador = Indicador::new(&config);
	let mut mail  = Mail::new( crate::config::autenticador().unwrap() );

	std::thread::Builder::new()
		.name( String::from("update-mail-indicator-thread") )
		.spawn(move || {
			let config_arc    = Arc::new( config );
			let indicador_arc = Arc::new( Mutex::new(indicador) );

			loop {
				let config_clone    = config_arc.clone();
				let indicador_clone = indicador_arc.clone();

				match mail.cant_mails_sin_leer() {
					Ok(count_mails_sin_leer) => {
						glib::source::idle_add( move || {
							indicador_clone.lock().unwrap().cambiar_icono(
								&config_clone,
								count_mails_sin_leer
							);

							return glib::source::Continue(false);
						});
					}
					Err(e) => {
						error!("{:?}", e);
					}
				}
				std::thread::sleep( std::time::Duration::from_secs(15) );
			}
		}).expect("Fallo al crear update-mail-indicator-thread");

	gtk::main();
}
