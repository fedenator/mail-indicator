use std::rc::{ Rc };
use std::cell::{ RefCell };

pub struct Tarea {
	pub nombre    : String,
	pub escritores: Vec< Rc< RefCell<Escritor> > >,
	pub subtareas : Vec< Rc< RefCell<Tarea> > >,
}

impl Tarea {
	pub fn new(
		nombre    : String,
		escritores: Vec< Rc< RefCell<Escritor> > >,
	) -> Tarea
	{
		return Tarea {
			nombre    : nombre,
			escritores: escritores,
			subtareas : Vec::new(),
		};
	}

	pub fn crear_subtarea(
		&mut self,
		nombre    : String,
		escritores: Vec< Rc< RefCell<Escritor> > >,
	) -> Rc< RefCell<Tarea> >
	{
		let subtarea = Rc::new( RefCell::new(Tarea::new(nombre, escritores) ) );
		self.subtareas.push( subtarea.clone() );
		return subtarea;
	}

	pub fn registrar(&mut self, debug_level: &DebugLevel, mensaje: &Mensaje) {
		for escritor in &self.escritores {
			escritor.borrow_mut().registrar(debug_level, mensaje);
		}
	}

}

pub struct Mensaje {
	pub texto: String,
}

pub enum DebugLevel {
	Trace, Info, Warn, Error
}

pub trait Escritor {
	fn registrar(&mut self, debug_level: &DebugLevel, mensaje: &Mensaje);
}

pub struct GestorTareas {
	pub tareas    : Vec< Rc< RefCell<Tarea> > >,
	pub escritores: Vec< Box<Escritor> >,
}

impl GestorTareas {
	pub fn new(escritores: Vec< Box<Escritor> >) -> Self {
		return GestorTareas {
			tareas    : Vec::new(),
			escritores: escritores,
		};
	}

	pub fn crear_tarea(
		&mut self,
		nombre    : String,
		escritores: Vec< Rc< RefCell<Escritor> > >,
	) -> Rc< RefCell<Tarea> >
	{
		let tarea = Rc::new( RefCell::new( Tarea::new(nombre, escritores) ) );

		self.tareas.push( tarea.clone() );

		return tarea;
	}
}
