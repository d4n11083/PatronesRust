//! Example of design pattern inspired from Head First Design Patterns
//! Tested with rust-1.3.0
//! @author Eliovir <http://github.com/~eliovir>


// Este va a ser nuestro Observador
trait Observer {
	fn update(&self);
}

// El observable memoriza a todos los observadores y les envía notificaciones
trait Observable<'a, T: Observer> {
	fn add_observer(&mut self, observer: &'a T);
	fn delete_observer(&mut self, observer: &'a T);
	fn notify_observers(&self);
}

//Acá se definen tanto el Observador como el Observable
struct Display {
	name: String,
}
struct Weather<'a, T:'a> {
	temperature: f64,
	observers: Vec<&'a T>
}
impl<'a> Weather<'a, Display> {
	fn set_temperature(&mut self, temperature: f64) {
		self.temperature = temperature;
		self.notify_observers();
	}
}
/*
 * Implementaciones
 */
impl Observer for Display {
	fn update(&self) {
		println!("Pantalla {} actualizada!", self.name);

	}
}
impl Display {
	fn new(name: String) -> Display {
		Display{name: name}
	}
}
impl std::cmp::PartialEq for Display {
	fn eq(&self, other: &Display) -> bool {
		self.name == other.name
	}
}
impl std::fmt::Display for Display {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Pantalla {}", self.name)
	}
}
impl<'a, T: Observer+PartialEq+std::fmt::Display> Observable<'a, T> for Weather<'a, T> {
	fn add_observer(&mut self, observer: &'a T) {
		println!("Añadir Observador({});", observer);
		self.observers.push(observer);
	}
	fn delete_observer(&mut self, observer: &'a T) {
		let mut index = 0;
		let mut found = false;
		for &obs in self.observers.iter() {
			if obs == observer {
				println!("Borrando Observador({});", observer);
				found = true;
				break;
			}
			index += 1;
		}
		if found {
			self.observers.remove(index);
		}
	}
	fn notify_observers(&self) {
		for &observer in self.observers.iter() {
			observer.update();
		}
	}
}

fn main() {

    println!( "\nVamos a crear las pantallas que van a ser los Observers" );
	let display = Display::new("Escritorio1".to_string());
	let display2 = Display::new("Escritorio2".to_string());

    println!("\nEl clima se va a inicializar con una temperatura de 19 grados. ");
	let mut weather = Weather{temperature: 19.0, observers: Vec::new()};

    println!("\nVamos a decirle a los Observadores que observen el clima.");
	weather.add_observer(&display);
	weather.add_observer(&display2);

    println!("\nVamos a cambiar la temperatura a 20 grados.");
	weather.set_temperature(20.0);

    println!("\nVamos a cambiar la temperatura a 21 grados.");
	weather.set_temperature(21.0);

    println!("\nVamos a cambiar la temperatura a 25 grados.");
    weather.set_temperature(25.0);

    println!("\nVamos a eliminar la Pantalla 2");
    weather.delete_observer(&display2);

    println!("\nVamos a cambiar la temperatura a 20 grados.");
    weather.set_temperature(20.0);

}
