use std::collections::HashMap;

fn main() {
    let mut capitals: HashMap<&str, &str> = HashMap::new();

    capitals.insert("Venezuela", "Caracas");
    capitals.insert("España", "Madrid");
    capitals.insert("Colombia", "Bogotá");
    capitals.insert("Argentina", "Buenos aires");

    for city in capitals {
        println!("La capital de {0} es {1}", city.0, city.1);
    }
}
