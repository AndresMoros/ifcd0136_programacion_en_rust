#[allow(unused)]
#[allow(dead_code)]
use rand::{RngExt, random_range};

#[derive(Debug)]
enum Direccion {
    Izquierda,
    Derecha,
}

#[derive(Debug)]
enum MovimientoJugador {
    Pasar,                              // Variante simple
    Correr(Direccion),                  // Variante de tupla
    Teletransportar { x: i32, y: i32 }, // Variante de Struct
}

enum Estaciones {
    Primavera,
    Verano,
    Otoño,
    Invierno,
}

fn main() {
    let jugador_nombre = "Kuker";

    // let x = ;
    // let y = ;

    let j_posicion: MovimientoJugador = MovimientoJugador::Teletransportar {
        x: rand(-50, 100),
        y: rand(0, 10),
    };

    let mover = MovimientoJugador::Correr(Direccion::Derecha);

    println!(
        "El jugador: {jugador_nombre}, está en la coordenada: {j_posicion:?} y se ha movido hacia: {mover:?}"
    );
}

fn rand(start: i32, end: i32) -> i32 {
    random_range(start..=end)
}
