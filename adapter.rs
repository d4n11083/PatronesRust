/*
 * Adapter Design Pattern
 * http://joshldavis.com/rust-design-patterns/adapter/
 */

/*
 * Primero definimos la estructura basica de un Cohete
 */
trait RocketShip {
    fn encender(&self);
    fn apagar(&self);
    fn despegar(&self);
    fn volar(&self);
}

/*
 * Se define la estructura basica de una nave de la NASA
 */
struct NASAShip;

/*
 * NASAShip implementa RocketShip para añadir funcionalidad
 */
impl RocketShip for NASAShip {
    fn encender(&self) {
        println!("Nave de la NASA está encendida.");
    }

    fn apagar(&self) {
        println!("Nave de la NASA está apagada");
    }

    fn despegar(&self) {
        println!("Nave de la NASA está despegando");
    }

    fn volar(&self) {
        println!("Nave de la NASA está volando.");
    }
}

/*
 * Nuestro astronauta no sabe utilizar la nave Spacex que es el modelo más avanzado.
 * Houston we have a prblem
 */
trait SpaceXShip {
    fn ignicion(&self);
    fn encendido(&self);
    fn apagado(&self);
    fn despegue(&self);
    fn vuelo(&self);
}

/*
 * Estructura Básica para nuestra nave Dragón SpaceX
 */
struct SpaceXDragon;

/*
 * Se implementa SpaceX para darle funcionalidad
 */
impl SpaceXShip for SpaceXDragon {
    fn ignicion(&self) {
        println!("ATENCIÓN... Ignición");
    }

    fn encendido(&self) {
        println!("ATENCIÓN... Encendiendo al Dragón");
    }

    fn apagado(&self) {
        println!("ATENCIÓN...Apagando al Dragón.");
    }

    fn despegue(&self) {
        println!("ATENCIÓN... Lanzamiento del Dragón en proceso");
    }

    fn vuelo(&self) {
        println!("TATENCIÓN... El Dragón está volando")
    }
}

/*
 *  Nuestra Nave Dragón SpaceX no implementa la interfaz del Cohete, vamos a crear el adapter
 */

/*
 * Vamos a crear un adapter para que cualquier cosa que implemente Nave SpaceX se pueda adaptar al Cohete
 */
struct SpaceXAdapter {
    ship: SpaceXDragon
}

/*
 * Adapter de SpaceX que añade la funcionalidad del Cohete a cualquier Nave de SpaceX
 */
impl RocketShip for SpaceXAdapter {
    fn encender(&self) {
        self.ship.ignicion();
        self.ship.encendido();
    }

    fn apagar(&self) {
        self.ship.apagado();
    }

    fn despegar(&self) {
        self.ship.despegue();
    }

    fn volar(&self) {
        self.ship.vuelo();
    }

}

/*
 * Función básica para pilotar naves que implementen Cohete.
 */
fn pilot<S: RocketShip>(ship: &S) {
    ship.encender();
    ship.despegar();
    ship.volar();
    ship.apagar();
    print!("\n");
}

fn main() {

    // Vamos a crear una nave de la Nasa
    println!("\nCreando el Saturno 5 ");
    let saturn5 = NASAShip;

    // Vanos a volar nuestro Saturno
    println!("\nPilotando el Saturno 5");
    pilot(&saturn5);

    // Vamos a crea un dragon
    println!("\nAhora vamos a crear un Dragón de SpaceX");
    let dragon = SpaceXDragon;




    //Nuestro piloto no reconoce esta nave,
    // pilot(&dragon); <--Da un error de compilación
    println!("\nNuestro piloto no sabe navegar está nave, vamos a utilizar el adapter");

    // Vamos a adaptar nuetsro dragón
    println!("\n....Adaptando nave....");
    let dragon_adapter = SpaceXAdapter {
        ship: dragon
    };

    //Ahora podemos pilotar la nave dragón
    println!("Pilotando el Dragón ahora ya adaptado.");
    pilot(&dragon_adapter);
}
