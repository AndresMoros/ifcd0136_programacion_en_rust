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
