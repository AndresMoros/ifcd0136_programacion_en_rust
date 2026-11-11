// ================= CONFIGURACIÓN DE EJECUCIÓN =================

#[allow(unused)]
#[allow(dead_code)]
use rand::{RngExt, random_range};

// ================= STRUCTS Y ENUMS =================
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

#[derive(Debug)]
enum Estaciones {
    Primavera,
    Verano,
    Otoño,
    Invierno,
}

enum ConnectionStates {
    Connected,
    Disconnected,
    Connecting,
}

// ================= FUNCION MAIN =================

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

    println!("{}", connection_state(ConnectionStates::Connecting));
}

// ================= FUNCIONES ADICIONALES =================

fn rand(start: i32, end: i32) -> i32 {
    random_range(start..=end)
}

fn connection_state(state: ConnectionStates) -> String {
    match state {
        ConnectionStates::Connected => String::from("Connected"),
        ConnectionStates::Disconnected => String::from("Disconnected"),
        ConnectionStates::Connecting => String::from("Connection in progress..."),
    }
}
