/*

 * Abstract Factory Design Pattern
 * http://joshldavis.com/rust-design-patterns/abstract-factory/
 */

/*
 * El rasgo principal que define a un teléfono
 */
trait Phone {
    fn call(&self); // Con la función de realizar una llamada
}

/*
 * Se define una tablet
 */
trait Tablet {
    fn play_games(&self); //Con la función de poder jugar a un videojuego
}

/*
 * Core trait qué define una fabrica dónde se van a generar teléfonos y tablets.
 */
trait Factory<P: Phone, T: Tablet> {
    fn new_phone(&self) -> P;
    fn new_tablet(&self) -> T;
}


/*
 * Acá se define que lo qué queremos hacer son productos Apple por ejemplo,
 * quéremos realizar un iPhone y un iPad.
 */

 /*
  *Aquí se crea la estructura iPhone que utiliza la definición de Phone (teléfono) qué se encuentra
  *más arriba.
  */
struct IPhone;

impl Phone for IPhone {
    //println!("iPhone creado");
    fn call(&self) {
        println!("Llamando desde un iPhone");
    }
}

/*
 *Al igual que con el iPhone acá se crea el iPad.
 */
struct IPad;

impl Tablet for IPad {
    //println!("iPad creado ");
    fn play_games(&self) {
        println!("Jugando en mi iPad");
    }
}

/*
 * Create AppleFactory and implement it for our Apple devices
 * En esta sección tenemos nuestra fábrica de Apple, implementada para crear
 * iPhones y iPads.
 */
struct AppleFactory;

impl Factory<IPhone, IPad> for AppleFactory {
    fn new_phone(&self) -> IPhone {
        println!("Fabricando un iPhone");
        return IPhone;
    }

    fn new_tablet(&self) -> IPad {
        println!("Fabricando un iPad");
        return IPad;
    }
}

/*
 * Ya tenemos una fábrica de Apple, ahora vamos a utilizar las definiciones del principio,
 * para crear una fábrica de Google.
 */

 /*
  *Acá creamos un Google Pixel 2
  */
struct Pixel2;

impl Phone for Pixel2 {
    //println!("Pixel 2 fabricado ");
    fn call(&self) {
        println!("Llamando desde mi Pixel 2");
    }
}

struct PixelBook;

impl Tablet for PixelBook {
    //println!("PixelBook creada");
    fn play_games(&self) {
        println!("Jugando en mi PixelBook");
    }
}

/*
 * Creamos nuestra fábrica de google.
 */
struct GoogleFactory;

impl Factory<Pixel2, PixelBook> for GoogleFactory {

    fn new_phone(&self) -> Pixel2 {
        println!("Fabricando teléfono Pixel 2");
        return Pixel2;
    }
    fn new_tablet(&self) -> PixelBook {
        println!("Fabricando PixelBook");
        return PixelBook;
    }
}


fn main() {
    // Primero vamos a crear nuestras dos fabricas.
    let apple = AppleFactory;
    let google = GoogleFactory;

    //Vamos a utilizar la misma interfaz para las fábricas.


    // Vamos a crear nuestro iphone
    println!("Quiero un iPhone!");
    let phone = apple.new_phone();
    phone.call();


    //Vamos a crear un Google Pixel 2
    let phone = google.new_phone();
    phone.call();


    //Ahora vamos a fabricar las tablets
    println!("Quiero un iPad");
    let tablet = apple.new_tablet();
    tablet.play_games();


    println!("Quiero un PixelBook");
    let tablet = google.new_tablet();
    tablet.play_games();
}
