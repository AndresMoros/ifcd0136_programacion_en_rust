fn main() {
    // Definir una closure multilinea
    let squared_sum = |x: i32, y: i32| {
        // Suma ambos parámetros
        let sum: i32 = x + y;

        // Calcula el cuadrado de los parámetros
        let result: i32 = sum * sum;

        result
    };

    // Llamar la closure
    let resultado = squared_sum(5, 3);

    println!("Resultado = {}", resultado);

    let discount = 0.10;

    // En Rust, mut es un modificador del enlace o variable en sí,
    // no del tipo de dato. Al escribir mut price: f64, le estás
    // diciendo al compilador: "crea una variable mutable llamada
    // price cuyo tipo es f64 a partir del valor que recibas".
    let apply_discount = |mut price: f64| {
        if price >= 20.0 {
            price -= price * discount
            // Las closures pueden acceder a variables de su ámbito
            // En este caso discount es una variable declarada dentro
            // de la funcion main()
        }
        price
    };

    let price = 24.0;

    let to_pay = apply_discount(price);

    println!("El precio es: {price}");
    println!("El total es: {to_pay}");

    // let other = apply_discount(32.0);
}
