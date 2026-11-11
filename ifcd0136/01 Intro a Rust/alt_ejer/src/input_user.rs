use std::io;

pub fn read_i32(input: &mut String) -> i32 {
    input.clear();

    io::stdin().read_line(input).expect("Error al leer");

    input.trim().parse().expect("NaN")
}

pub fn read_str(input: &mut String) -> String {
    input.clear();

    io::stdin().read_line(input).expect("Error al leer");

    input.trim().parse().expect("Error al convertir en cadena")
}
