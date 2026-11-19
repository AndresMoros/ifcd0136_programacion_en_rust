mod utils;
use std::iter;

// CARGA DE MÓDULOS EN EL SCOPE
use utils::*;

fn main() {
    println!(
        "Generar un vector de 300 numeros enteros (Vec<i32>) aleatorios,\ny mostrar cuantos son negativos, positivos y ceros (0 es positivo).\nHacerlo mediante iteradores"
    );

    let mut numbers: Vec<i32> = vec![];

    let mut zero = vec![];
    let mut positive = vec![];
    let mut negative = vec![];

    for n in 1..=300 {
        let rn = rand_num(-300, 300);
        numbers.push(rn);
        match rn {
            n if n > 0 => positive.push(n),
            n if n < 0 => negative.push(n),
            _ => zero.push(n),
        }
    }

    // println!("Numeros positivos: {:?}", positive.iter().count());
    // println!("Numeros negativos: {:?}", negative.iter().count());
    // println!("Numeros 0: {:?}", zero.iter().count());

    let iter_positive = numbers.iter().filter(|n| **n > 0);
    let iter_negative = numbers.iter().filter(|n| **n < 0);
    let iter_zero = numbers.iter().filter(|n| **n == 0);

    println!("Numeros positivos: {:?}", iter_positive.count());
    println!("Numeros negativos: {:?}", iter_negative.count());
    println!("Numeros 0: {:?}", iter_zero.count());
}
