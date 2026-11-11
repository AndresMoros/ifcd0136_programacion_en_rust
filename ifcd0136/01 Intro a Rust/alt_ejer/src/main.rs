mod calulator;
mod input_user;

use calulator::Calculator;
use input_user::read_i32;
use input_user::read_str;

fn main() {
    ejer1()
}

fn ejer1() {
    let mut input: String = String::new();

    println!("Escribe el primer número");
    let n1 = read_i32(&mut input);
    println!("Escribe el segundo número");
    let n2 = read_i32(&mut input);
    println!(
        "¿Qué operación quieres hacer?
        Suma: +
        Resta: -
        Multiplicación: *
        Devisión: /"
    );
    let op = read_str(&mut input);

    let result = Calculator::new(n1, n2, op.clone());

    if result.calc().1.contains("Error") {
        println!("{:?}", result.calc().1)
    } else {
        println!(
            "El resultado de {n1} {0} {n2} es: {1:?}",
            op,
            result.calc().0
        )
    }
}
