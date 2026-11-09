use rand::RngExt;
use std::io;
fn main() {
    // EJECUCIÓN DEL EJERCICIO 1
    println!(
    "Ejercicio 1:
    1. Rellenar un array de 5 números de tipo entero. Una vez relleno, pedir un número y contestar al usuario si ese número está en el array.
    ¿Quieres ejecutar este ejercicio? S/N (por defecto N)");

    let x = choose_check();

    if x {
        n_guess();
    }

    // EJECUCIÓN DEL EJERCICIO 2
    println!(
    "Ejercicio 2: 
    Programa para comprar asientos de un autobús de 10 plazas, este marca los asientos ocupados con un “1” y los libres con un “0”. Solicitar al comprador que plaza quiere (del 1 al 10) y si el asiento ya se ha vendido, el programa lo señalará, si no se ha vendido la plaza quedará marcada con un “1” y le deseamos “Feliz viaje”.
    ¿Quieres ejecutar este ejercicio? S/N (por defecto N)");

    bus_ticket();
}

fn bus_ticket() {
    // 2. Programa para comprar asientos de un autobús de 10 plazas, este marca los asientos
    // ocupados con un “1” y los libres con un “0”.
    // Solicitar al comprador que plaza quiere (del 1 al 10) y si el asiento ya se ha vendido, el
    // programa lo señalará, si no se ha vendido la plaza quedará marcada con un “1” y le deseamos
    // “Feliz viaje”.
    // 3. Almacenar 300 números aleatorios en un vector, imprimir cuantos son ceros, cuantos son
    // negativos, cuantos positivos.
    // 0 = libre, 1 = ocupado
    let mut bus_seats: [i32; 10] = [0; 10];

    for seats in &mut bus_seats {
        *seats = n_rand(0, 1);
    }

    loop {
        // Mostrar el estado del autobús
        println!("\nEstado del autobús (0 = libre, 1 = ocupado):");
        for (i, seat) in bus_seats.iter().enumerate() {
            println!("Asiento {}: {}   ", i + 1, seat);
        }

        // Si no quedan plazas libres, terminamos
        if bus_seats.iter().all(|&s| s == 1) {
            println!("El autobús está completo. ¡Buen viaje a todos!");
            break;
        }

        println!("¿Qué plaza quieres? (del 1 al 10, 0 para salir):");
        let seat_num = read_num_i32();

        if seat_num == 0 {
            println!("Hasta luego.");
            break;
        }

        if seat_num < 1 || seat_num > 10 {
            println!("Esa plaza no existe. Elige del 1 al 10.");
            continue;
        }

        let idx = (seat_num - 1) as usize;

        if bus_seats[idx] == 1 {
            println!("La plaza {seat_num} ya está vendida.");
        } else {
            bus_seats[idx] = 1;
            println!("¡Feliz viaje! Plaza {seat_num} reservada.");
        }
    }
}

fn n_guess() {
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

/// Devuelve un número al alzar en el rango entre `start` y ´end´ (inclusivo)
/// # Parámetros
/// - `start`: `i32`
/// - `end`: `i32`
/// # Devuelve
/// `i32`
fn n_rand(start: i32, end: i32) -> i32 {
    rand::rng().random_range(start..=end)
}

/// Lee la entrada del usuario y la convierte a un `i32`
/// # Devuelve
/// `i32`
fn read_num_i32() -> i32 {
    let mut txt = String::new();

    io::stdin().read_line(&mut txt).expect("Error");

    let n: i32 = txt.trim().parse().expect("Eso no es un número");

    n
}

/// Pregunta al usuario por consola (s/n) y devuelve `true` si responde "s"
/// # Devuelve
/// `true` si `respond` es "s"
/// `false` si `respond` es "n"
fn choose_check() -> bool {
    let mut respond: String = String::new();

    io::stdin().read_line(&mut respond).expect("Error");

    if respond.trim().to_lowercase() == "s" {
        respond.clear();
        true
    } else if respond.trim().to_lowercase() == "n" {
        respond.clear();
        println!("Ejercicio saltado\n##############################\n");
        false
    } else {
        respond.clear();
        println!("Ejercicio saltado\n##############################\n");
        false
    }
}
