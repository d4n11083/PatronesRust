
use std::fmt;

#[derive(Debug)]
enum Lenguaje{
    Ingles,
    Espanol,
}

#[derive(Debug)]
struct Greeter{
 lenguaje : Lenguaje,
}

/*
 *Se Crea un display para greeter
 */
impl fmt::Display for Greeter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let saludo = match self.lenguaje{
            Lenguaje::Ingles => "Hello",
            Lenguaje::Espanol => "Hola",
        };
        write!(f, "{} Rust", saludo)
    }
}

/*
 * Se implementa el Greeter con el idioma inglés como default
 * pero se puede construir con el lenguaje que se desea, en este caso
 * inglés o español.
 */
impl Greeter{
    fn new()-> Greeter{
        Greeter  {
            lenguaje : Lenguaje::Ingles,
        }
    }

    fn with_language(mut self, lenguaje : Lenguaje) -> Greeter{
        self.lenguaje = lenguaje;
        self
    }
}


fn main(){
    
    //Se crea un greeter en español
    let greeter = Greeter::new().with_language(Lenguaje:: Espanol);
    assert_eq!(format!("{}", greeter), "Hola Rust");
    println!("{:?}", greeter);

    //Se crea un greeter en inglés
    let greeter2 = Greeter::new().with_language(Lenguaje:: Ingles);
    assert_eq!(format!("{}", greeter2), "Hello Rust");
    println!("{:?}", greeter2);
}
