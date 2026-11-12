#[allow(unused)]
#[allow(dead_code)]
// DECLARACIÓN DE MÓDULOS
mod calulator;
mod input_user;
// CARGA DE MÓDULOS EN EL SCOPE
use calulator::Calculator;
use input_user::read_f64;
use input_user::read_str;

fn main() {
    ejer1();

    ejer2();

    ejer5();
}

fn ejer1() {
    println!(
        "1. Recibo por teclado 2 números y un operador que podrá ser (+, - , *, /) realizar la operación que corresponda y calcular el resultado."
    );
    let mut input: String = String::new();

    println!("Escribe el primer número");
    let n1: f64 = read_f64(&mut input);

    println!("Escribe el segundo número");
    let n2: f64 = read_f64(&mut input);

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
            "El resultado de {n1} {0} {n2} es: {1:2?}",
            op,
            result.calc().0
        )
    }
}

fn ejer2() {
    println!("2. Introducir un número y visualizar si es par o impar.");

    let mut input: String = String::new();

    let n: f64 = read_f64(&mut input);

    println!(
        "El número: {n} es {}.",
        if n % 2.0 == 0.0 { "par" } else { "impar" }
    )
}

fn ejer5() {
    println!(
        "5. Elaborar un programa que muestre el precio de un billete de autobús, que se calcula en base a los kilómetros de trayecto (0.30 Euros por Km), pero si el recorrido supera los 80 Km se aplica un 15 % de descuento y que si el trayecto se realiza en día laborable (‘L’) hay un 5 % de descuento respecto si es día festivo (‘F’). Datos que introducir: kilómetros de recorrido y tipo de día (L-F)."
    );

    // 1. Declaramos las variables / constantes que vamos a utilizar
    const PRECIO_KM: f64 = 0.30;
    const DISTANCIA_MINIMA: f64 = 80.0;
    const DESCUENTO_DISTANCIA: f64 = 0.15;
    const DESCUENTO_DIA: f64 = 0.05;
    let precio_billete: f64;

    // 2. Pedimos al usuario que indique cuántos kilómetros va a recorrer y qué día
    let mut input: String = String::new();

    println!("Indica cuántos kilometros vas a viajar:");
    let km = read_f64(&mut input);
    println!("Indica qué día vas a viajar (L: día laborable, F: dia festivo):");
    let dia = read_str(&mut input);

    precio_billete = km * PRECIO_KM;

    // 3. Calculamos los descuentos de ser el caso
    let desc_km = if km > DISTANCIA_MINIMA {
        println!("¡Felicidades, tienes un descuento por distancia!");
        precio_billete * DESCUENTO_DISTANCIA
    } else {
        0.0
    };
    let desc_dia = if dia == String::from("L") {
        println!("¡Felicidades, tienes un descuento por el día!");
        precio_billete * DESCUENTO_DIA
    } else {
        0.0
    };

    // 4. Aplicamos los descuentos:
    let precio_billete_final = precio_billete - desc_km - desc_dia;

    // 5. Imprimimos todo
    println!("----------------------------------------");
    println!("=========== DATOS DE BILLETE ===========");
    println!("----------------------------------------");
    println!("DISTANCIA: {km};");
    println!("DIA: {dia}");
    println!("----------------------------------------");
    println!("============== DESCUENTOS: =============");
    println!("----------------------------------------");
    println!("DESCUENTO POR KM (+80 km): 15% - {:.2}", desc_km);
    println!("DESCUENTO POR DIA (L): 5% - {}", desc_dia);
    println!("----------------------------------------");
    println!("================ TOTAL =================");
    println!("----------------------------------------");
    println!("PRECIO ({km} km * € {PRECIO_KM}): {precio_billete}");
    println!("PRECIO FINAL: ({precio_billete} - (€ {desc_km:.2} + € {desc_dia}))");
    println!("TOTAL A PAGAR: {precio_billete_final:.2}");
}
