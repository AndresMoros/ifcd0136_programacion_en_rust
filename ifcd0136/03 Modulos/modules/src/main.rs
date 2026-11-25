mod geometry {
    // Las funciones del modulo que queramos usar fuera de este debe
    pub fn square_area(side: f64) -> f64 {
        side * side
    }

    pub struct Square {
        side: f64,
        area: f64,
    }

    impl Square {
        pub fn new(side: f64) -> Self {
            Square {
                side: side,
                area: square_area(side),
            }
        }

        pub fn area(&self) -> f64 {
            self.area
        }

        pub fn side(&self) -> f64 {
            self.side
        }
    }
}

use crate::geometry::*;

fn main() {
    let figure: Square = Square::new(64.4);

    println!("Figure side: {}", figure.side());
    println!("Figure area: {:.2}", figure.area());
}
