#[allow(unused)]
#[allow(unused_attributes)]
#[allow(unused_crate_dependencies)]
#[allow(unused_variables)]
#[derive(Debug)]

struct CoordenadasInt<T> {
    x: T,
    y: T,
}

// En caso de querer dos tipos genéricos distintos debo agregar otra letra
struct CoordenadasFloat<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Punto<T, U> {
    x: T,
    y: U,
}

impl<T, U> Punto<T, U> {
    fn coordenadas(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }
    fn establecer_x(&mut self, x: T) {
        self.x = x;
    }
    fn establecer_y(&mut self, y: U) {
        self.y = y;
    }
}

fn main() {
    let coordenadas_entero = CoordenadasInt { x: 32, y: 100 };
    let coordenadas_flotante = CoordenadasFloat { x: -100.98, y: 87 };
    let mut punto_impl = Punto { x: 50, y: -50 };

    println!(
        "Los valores de la coordenadas son: {:?}",
        punto_impl.coordenadas()
    );

    punto_impl.establecer_x(8);
    punto_impl.establecer_y(10);

    println!(
        "Los valores de la coordenadas ahora son: {:?}",
        punto_impl.coordenadas()
    );
}
