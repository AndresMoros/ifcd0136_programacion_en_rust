use std::println;


fn main() {
    let ciudades = ["Madrid", "Caceres", "Valencia"];

    for ciudad in ciudades {
        println!("Ciudad en array: {ciudad}");
    }

    // Cambiar el valor de los elementos de un array requiere hacerlo mutable con mut

    let mut prices = [10.0, 12.0, 32.0, 45.0];

    println!("Precios completos: {:#?}", prices);

    // Se debe trabajar con una referencia en memoria del array, para ello se usa &mut
    for price in &mut prices {
        // se debe usar * para indicar que se está editando el dato guardado en el puntero de memoria de price
        *price *= 0.95;
    }

    println!("Precios con 5% descuento: {:#?}", prices);

    // ============================================================
    // Con {:p} se pueden imprimir los punteros de memoria que usa el dato

    let mut dato = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for x in &mut dato {
        println!("Elemento ) -> Memoria: {:p}", x);
    }

}
