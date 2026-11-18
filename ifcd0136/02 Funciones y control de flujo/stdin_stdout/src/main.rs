use std::env;

fn main() {
    let count = env::args().count();
    let args: Vec<String> = env::args().collect();

    println!("Datos recibidos {count}");

    for arg in &args {
        println!("{arg}");
    }
}
