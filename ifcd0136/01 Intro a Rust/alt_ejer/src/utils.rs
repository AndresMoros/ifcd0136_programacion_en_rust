#[allow(unused)]
#[allow(dead_code)]
use std::io;

/// Lee input del usuario y devuelve un `f64`
/// # Param
/// - `input`: &mut String
/// # Devuelve
/// - `n`: f64
pub fn read_f64(input: &mut String) -> f64 {
    let n: f64;
    loop {
        input.clear();
        io::stdin().read_line(input).expect("Error al leer");
        match input.trim().replace(',', ".").parse() {
            Ok(num) => {
                n = num;
                break;
            }
            Err(_e) => {
                println!("Introduce un número");
            }
        }
    }
    n
}

// Lee input del usuario y devuelve un `i32`
// # Param
// - `input`: &mut String
// # Devuelve
// - `n`: i32
// pub fn read_i32(input: &mut String) -> i32 {
//     let n: i32;
//     loop {
//         input.clear();
//         io::stdin().read_line(input).expect("Error al leer");
//         match input.trim().replace(',', ".").parse() {
//             Ok(num) => {
//                 n = num;
//                 break;
//             }
//             Err(_e) => {
//                 println!("Introduce un número");
//             }
//         }
//     }
//     n
// }

/// Lee input del usuario y devuelve un `String`
/// # Param
/// - `input`: &mut String
/// # Devuelve
/// - `input`: String
pub fn read_str(input: &mut String) -> String {
    input.clear();
    io::stdin().read_line(input).expect("Error al leer");
    input.trim().parse().expect("Error al convertir en cadena")
}
// 1. Recibo por teclado 2 números y un operador que podrá ser (+, - , *, /)
// Realizar la operación que corresponda y calcular el resultado.
#[derive(Debug)]
pub struct Calculator {
    x: f64,
    y: f64,
    operator: String,
}

impl Calculator {
    /// Crea un nuevo Struct Calculator
    /// # Parametros
    /// - `x`: i32
    /// - `y`: i32
    /// - `op`: String | Operador para hacer los cálculos
    pub fn new(x: f64, y: f64, op: String) -> Self {
        Calculator {
            x: x,
            y: y,
            operator: op,
        }
    }

    /// Devuelve la el resultado la operación entre dos números: `x` & `y`
    /// # Parametros
    /// - `y`: i32
    /// - `x`: i32
    /// # Devuelve
    /// `(Resultado en i32`, `Estado de la operación`)
    pub fn calc(&self) -> (f64, &str) {
        match self.operator.as_str() {
            "+" => (self.x + self.y, "Ok"),
            "-" => (self.x - self.y, "Ok"),
            "*" => (self.x * self.y, "Ok"),
            "/" => {
                if self.y == 0.0 {
                    (0.0, "No se pueden dividir números entre 0")
                } else {
                    (self.x / self.y, "Ok")
                }
            }
            _ => (0.0, "Error, operador NO válido"),
        }
    }
}

// ########################## LOGICA PARA EL CAJERO AUTOMATICO ##########################
/// struct Person
/// # Propiedades:
/// - `name`: String
/// - `last_name`: String
/// - `dni`: String
/// # Métodos:
/// - `new`: constructor
/// - `get_name`
/// - `get_last_name`
/// - `get_dni`
#[derive(Debug)]
struct Person {
    name: String,
    last_name: String,
    dni: String,
}

impl Person {
    /// Función constructora del struct `Person`
    /// # Params:
    /// - `name`: String
    /// - `last_name`: String
    /// - `dni`: String
    /// # Devuelve
    /// - `Person` struct
    pub fn new(&mut self, name: String, last_name: String, dni: String) -> Self {
        Person {
            name: name,
            last_name: last_name,
            dni: dni,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn get_dni(&self) -> String {
        self.dni.clone()
    }
}

/// struct Client
/// # Propiedades:
/// - `personal`: Person
/// - `password`: String
/// - `balance`: String
/// # Métodos:
/// - `new`: constructor
/// - `get_name`
/// - `get_last_name`
/// - `get_dni`
struct Client {
    personal: Person,
    password: String,
    balance: f64,
}

impl Client {
    /// Función constructora del struct `Client`
    /// # Params:
    /// - `per`: Person | Datos personales
    /// - `pass`: String | Contraseña
    /// - `bal`: f64 | Saldo disponible
    /// # Devuelve
    /// - `Client` struct
    pub fn new(&mut self, per: Person, pass: String, bal: f64) -> Self {
        Client {
            personal: per,
            password: pass,
            balance: bal,
        }
    }
    /// Obtener el saldo disponible
    /// # Param
    /// - `self`
    /// # Devuelve
    /// - `self.balance`: f64
    pub fn get_bal(self) -> f64 {
        self.balance
    }
    /// Establecer el saldo disponible
    /// # Param
    /// - `&mut self`: &mut Client
    pub fn set_bal(&mut self, amount: f64) {
        self.balance = amount;
    }

    // pub fn get_name(self) {
    //     self.name
    // }
}
/// Estructura de cajero debe poder:
/// - Verificar que el usuario existe
/// - Dar feedback al usuario
/// - Verificar si la contraseña es correcta
/// - Mostrar el saldo en cualquier momento
/// - Hacer  
struct Cashpoint {}

impl Cashpoint {
    /// Permite acceder al usuario a la interfaz del cajero
    pub fn login(self, clients: [Client; 10], dni: String, pass: String) -> bool {
        const MAX_ATTEMPS: u8 = 3;
        let mut attemp = 0;
        let mut input: String = String::new();
        Self::message(&format!("Introduce tu DNI y tu contraseña:"));
        loop {
            // Damos la bienvenida al usuario para que se identifique
            Self::message(&format!("DNI:"));
            // Recogemos el DNI del usuario
            let mut stdin_dni = read_str(&mut input);
            // Debemos compararlo con los clientes que tenemos en un array
        }
        false
    }

    pub fn manage_users(users: [Client; 10], username: String) -> bool {
        for user in &users {
            if username == user.personal.get_dni() {
                break;
            }
            return false;
        }
        true
    }

    /// Imprime un mensaje dependiendo de la función lo use
    /// # Param:
    /// `msg`: &str con mensaje a devolver
    pub fn message(msg: &str) {
        println!("{msg}");
    }
}
