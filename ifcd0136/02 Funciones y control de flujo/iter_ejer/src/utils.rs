use rand::{RngExt, random};
use std::fmt::format;
use std::io;

#[allow(unused)]
#[allow(dead_code)]

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

/// Lee input del usuario y devuelve un `i32`
/// # Param
/// - `input`: &mut String
/// # Devuelve
/// - `n`: i32
pub fn read_i32(input: &mut String) -> i32 {
    let n: i32;
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

/// Genera un número aleatorio en el rango especificado
/// # Params
/// - `start`: i32 | El numero desde el que se empieza el rango
/// - `end`: i32 | El numero desde el que se empieza el rango (incluido)
/// Devuelve:
/// i32
pub fn rand_num(start: i32, end: i32) -> i32 {
    rand::random_range(start..=end)
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
pub struct Person {
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
    pub fn new(name: String, last_name: String, dni: String) -> Self {
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
#[derive(Debug)]
pub struct Client {
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
    pub fn new(per: Person, pass: String, bal: f64) -> Self {
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
    pub fn get_bal(&self) -> f64 {
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
/// Estructura de cajero:
/// - Dibujar el menú
/// - Dar feedback al usuario
/// - Realizar varias operaciones por sesión
/// - Ingresar dinero
/// - Retirar dinero
/// # Implementaciones:
/// - `check_balance`: ver el saldo del usuario
/// - `enter_money`: depositar dinero en la cuenta del usuario
/// - `withdraw_money`: retirar dinero en la cuenta del usuario
pub struct Cashpoint {}

impl Cashpoint {
    /// Dibuja el menú inicial del cajero

    pub fn menu(user: &mut Client) {
        loop {
            println!("\n--- Opciones cajero ---");
            println!("1. Consultar saldo");
            println!("2. Ingresar");
            println!("3. Retirar efectivo");
            println!("4. Salir");
            println!("Seleccionar una opcion: ");

            let mut input = String::new();
            let option = read_i32(&mut input);

            // Controlamos la opción que escoge el usuario

            match option {
                4 => {
                    println!("Hasta luego, {}", user.personal.get_name());
                    break; // Nos salimos del bucle solo si el usuario así lo desea, podemos realiza varias operaciones en una sola sesión
                }
                1 => {
                    println!("Su saldo es: € {}", Self::check_balance(user));
                }
                2 => {
                    println!("Saldo a ingresar: ");
                    let amount = read_f64(&mut input);
                    Self::enter_money(amount, user);
                    println!("Saldo ingresado: € {amount}");
                    println!("Su saldo es: € {}", Self::check_balance(user));
                }
                3 => {
                    println!("Saldo a retirar: ");
                    let amount = read_f64(&mut input);
                    if Self::withdraw_money(amount, user) {
                        println!("Operación exitosa.");
                        println!("Retirado: € {amount}");
                        println!("Su saldo es: € {}", Self::check_balance(user));
                    } else {
                        println!("Operación fallida.");
                        println!("No puedes retirar más dinero del que tienes.");
                        println!("Su saldo es: € {}", Self::check_balance(user));
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    /// Ingresa saldo
    /// # Params:
    /// - `amount`: f64 (cantidad de dinero)
    /// - `user`: &mut Client (referencia mutable para modificar el saldo)
    pub fn enter_money(amount: f64, user: &mut Client) {
        &mut user.set_bal(user.get_bal() + amount);
    }

    /// Devuelve el saldo disponible en la cuenta del usuario
    /// # Params:
    /// - `client`: &Client (cliente a quien se le verifica el saldo)
    pub fn check_balance(user: &Client) -> f64 {
        user.get_bal()
    }

    /// Retirar saldo
    /// # Params:
    /// - `amount`: f64 (cantidad de dinero)
    /// - `user`: &mut Client (referencia mutable para modificar el saldo)
    /// # Devuelve
    /// - `bool`: "true" si la operación es exitosa, "false" si falla
    pub fn withdraw_money(amount: f64, user: &mut Client) -> bool {
        if amount > user.get_bal() {
            false
        } else {
            &mut user.set_bal(user.get_bal() - amount);
            true
        }
    }
}
