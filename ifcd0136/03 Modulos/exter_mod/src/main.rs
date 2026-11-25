mod store;

use store::*;

use crate::store::item::*;

fn main() {
    let pencil = Item::new(String::from("Lapiz"), Categories::Accesories, 56.0);
}
