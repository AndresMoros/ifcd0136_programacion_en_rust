fn main() {
    // DECLARACIÓN DE VARIABLES
    // (Más información en 03 Variables y operadores.md)

    // INMUTABLE
    let precio = 100;       // por defecto las variables son inmutables, es decir no puede cambiarse
    println!("Variable original:");
    println!("Precio: {precio}");

    // precio = 250;
    /*
    error[E0384]: no se puede asignar un segundo valor la variable inmutable `precio`
    --> src\bin\variables.rs:10:5
   |
 6 |     let precio = 100;
   |         ------ primera asignación a `precio`
    ...
10 |     precio = 250;
   |     ^^^^^^^^^^^^ no se puede asignar un segundo valor la variable inmutable
   */

    // Shadowing (ensombramiento)
    let precio = "200";    // el shadowing es redeclarar una variable
    println!("Variable ensombrecida (cambio de tipo):");
    println!("Precio: {precio}");

    {
        let precio = 1000;
        println!("
        La manzana cuesta (dentro de otro scope): {precio}
        ");
    }   // el valor de precio es 1000 solo dentro de este ámbito
        // fuera vuelve a valer 200 como cadena de texto

    println!("El valor de la variable precio es: {precio}"); // imprime "200"

    println!("\n===========================\n");

    // MUTABLE
    let mut fruta = "Manzana"; //usando la palabra clave `mut` se puede volver una variable mutable

    println!("La fruta es: {fruta}");

    fruta = "Melocotón";
    // fruta = 100; // NO se puede cambiar el tipo, ya que Rust ya infirió un tipo a `fruta` la primera vez que era un &str

    println!("La fruta es (mutada): {fruta}");

}