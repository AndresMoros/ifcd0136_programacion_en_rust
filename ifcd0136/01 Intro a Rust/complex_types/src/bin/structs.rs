#[derive(Debug)]
enum OrentacionSexual {
    Masculino,
    Femenino,
    Otro,
}

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

struct Trabajador {
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
}
