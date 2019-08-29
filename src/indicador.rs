use std::sync::{ Arc, Mutex };

use gtk::{ MenuItemExt, MenuShellExt, WidgetExt };

use libappindicator::{ AppIndicator, AppIndicatorStatus };

use crate::config::{ Config };

/// Indicador en la barra de estado
#[derive(Clone)]
pub struct Indicador {
	app_indicator: Arc< Mutex<AppIndicator> >,
	menu         : gtk::Menu,
}

unsafe impl Send for Indicador {}
unsafe impl Sync for Indicador {}

impl Indicador {
	pub fn new(config: &Config) -> Self {
		let mut app_indicator = AppIndicator::new("Mail-indicator", "");
		app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);

		let app_indicator_rc = Arc::new( Mutex::new(app_indicator) );

		let mut indicador = Indicador {
			app_indicator: app_indicator_rc.clone(),
			menu         : crear_menu(),
		};

		indicador.cambiar_icono(&config, 0_u32);
		indicador.app_indicator.lock().unwrap().set_menu(&mut indicador.menu);

		return indicador;
	}

	pub fn cambiar_icono(&mut self, config: &Config, numero: u32) {
		cambiar_icono_app_indicator(&mut self.app_indicator.lock().unwrap(), config, numero);
	}
}

#[inline]
fn crear_menu() -> gtk::Menu {
	let menu = gtk::Menu::new();

	let opcion_salir = gtk::ImageMenuItem::new_from_stock("Cerrar", None);
	opcion_salir.connect_activate(|_| {
		gtk::main_quit();
	});
	menu.append(&opcion_salir);

	menu.show_all();

	return menu;
}

//Cambia el icono del indicador por el icono que corresponda al numero dado
fn cambiar_icono_app_indicator(
	app_indicator: &mut AppIndicator,
	config       : &Config,
	mut numero   : u32
)
{
	// No hay iconos mas grandes que 9
	if numero > 9 {
		numero = 9;
	}

	app_indicator.set_icon_full(
		config
			.carpeta_assets
			.join  ( format!("icon_{}.png", numero) )
			.to_str()
			.unwrap(),
		"icon"
	);
}
