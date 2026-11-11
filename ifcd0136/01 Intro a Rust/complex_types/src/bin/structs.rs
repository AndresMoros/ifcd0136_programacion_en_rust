#[allow(unused_unsafe)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused)]
#[allow(dead_code)]
// Enumeraciones y estructuras
#[derive(Debug)]
enum OrentacionSexual {
    Masculino,
    Femenino,
    Otro,
}

#[derive(Debug)]
enum Profesion {
    Programador,
    Analista,
    Desarrollador,
    Diseñador,
}

#[derive(Debug)]
struct Persona {
    nombre: String,
    nacimiento: u16,
    edad: u8,
    sexo: OrentacionSexual,
    es_adulto: bool,
}

#[derive(Debug)]
struct Trabajador {
    persona: Persona,
    puesto: Profesion,
    salario: u16,
}

impl Persona {
    fn new(nombre: String, nacimiento: u16, sexo: OrentacionSexual) -> Self {
        let edad = 2026 - nacimiento;
        Persona {
            nombre,
            nacimiento,
            edad: edad as u8,
            sexo,
            es_adulto: edad >= 18,
        }
    }

    /// Imprime los datos básicos del usuario
    /// # Params:
    /// `&self`
    fn print_data(&self) {
        println!(
            "Hola, mi nombre es {0:?}, tengo {1:?} años, y nací en el año {2:?}",
            &self.nombre, self.edad, self.nacimiento
        )
    }

    /// Devuelve la orientación del usuario
    /// # Params:
    /// `&self`
    /// # Devuelve
    /// `enum OrientacionSexual` del usuario
    //  Debido a que mi `enum OrientacionSexual` no implemento el trait Copy, debo utilizar una referencia
    fn get_gender(&self) -> &OrentacionSexual {
        &self.sexo
    }

    /// Devuelve un `bool` respecto si el usuario es adulto o menor de edad
    /// # Params:
    /// `&self`
    /// # Devuelve
    /// `bool`
    fn is_adult(&self) -> bool {
        self.es_adulto
    }
}

impl Trabajador {
    fn new(persona: Persona, puesto: Profesion, salario: u16) -> Self {
        Trabajador {
            persona,
            puesto,
            salario,
        }
    }
}

fn main() {
    let andres = Persona::new(
        String::from("Andrés Moros"),
        2003,
        OrentacionSexual::Masculino,
    );

    println!("Los datos de la persona son: {andres:#?}");

    andres.print_data();

    println!(
        "Sexo: {0:?} | ¿Es adulto?: {1}",
        andres.get_gender(),
        if andres.is_adult() { "Sí" } else { "No" },
    );

    // let sergio = Persona::new(
    //     String::from("Sergio Gutierrez"),
    //     1999,
    //     OrentacionSexual::Otro,
    // );

    let trabajador = Trabajador::new(
        Persona::new(
            String::from("Sergio Gutierrez"),
            1999,
            OrentacionSexual::Otro,
        ),
        Profesion::Programador,
        1280,
    );

    println!("Trabajador: {trabajador:?}");
}
