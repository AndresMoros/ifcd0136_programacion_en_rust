pub enum Categories {
    Sports,
    Clothes,
    Games,
    Accesories,
}

pub struct Item {
    name: String,
    category: Categories,
    price: f64,
}

impl Item {
    pub fn new(nombre: String, categoria: Categories, precio: f64) -> Self {
        Item {
            name: nombre,
            category: categoria,
            price: precio,
        }
    }
}
