use rand::RngExt;
use std::io;
fn main() {
    // 1. Rellenar un array de 5 números de tipo entero. Una vez relleno, pedir un número y contestar
    // al usuario si ese número está en el array.

    let mut int_arr: [i32; 10] = [0; 10];

    println!("---------------------------------------------------");
    println!("Array sin numeros aleatorios:");
    println!("---------------------------------------------------");

    println!("{:#?}", int_arr);

    for int in &mut int_arr {
        *int = n_rand(1, 100);
    }

    println!("---------------------------------------------------");
    println!("Array con numeros aleatorios:");
    println!("---------------------------------------------------");

    println!("{:#?}", int_arr);

    println!("---------------------------------------------------");
    println!("Escribe un número:");

    let user_input = read_num_i32();

    println!("El usuario escribió: {user_input}");
    println!("---------------------------------------------------");

    let mut respond: bool = false;

    for n in int_arr {
        if user_input == n {
            respond = true;
            break;
        }
    }

    println!("---------------------------------------------------");
    println!(
        "¿Incluye el array ese número?: {}",
        if respond { "Sí" } else { "No" }
    ); // Simulación de un operator ternario
    println!("---------------------------------------------------");
}

// 2. Programa para comprar asientos de un autobús de 10 plazas, este marca los asientos 
// ocupados con un “1” y los libres con un “0”. 
// Solicitar al comprador que plaza quiere (del 1 al 10) y si el asiento ya se ha vendido, el 
// programa lo señalará, si no se ha vendido la plaza quedará marcada con un “1” y le deseamos 
// “Feliz viaje”. 
// 3. Almacenar 300 números aleatorios en un vector, imprimir cuantos son ceros, cuantos son 
// negativos, cuantos positivos. 

fn n_rand(start: i32, end: i32) -> i32 {
    rand::rng().random_range(start..=end)
}

fn read_num_i32() -> i32 {
    let mut txt = String::new();

    io::stdin().read_line(&mut txt).expect("Error");

    let n: i32 = txt.trim().parse().expect("Eso no es un número");

    n
}
