fn main() {

    // Recorrer array y mostrar datos

    println!("Recorrer array y mostrar datos");

    let langs: [&str; 4] = ["Java", "Rust", "C++", "Python"];

    for i in langs {
        print!("{i} ");
    }

    println!("\n\nArray destructurado");

    // Desestructurar array

    let [
        lang1,
        lang2,
        lang3,
        lang4
        ] = langs;

    println!("{lang1}, {lang2}, {lang3}, {lang4}");

    // Modificar datos del array (con refenrecias y punteros)

    println!("\nAcceso y modificación de valores");

    // Debe declararse como mutable (mut)

    let mut langs: [&str; 4] = ["Java", "Rust", "C++", "Python"];

    for i in &mut langs { // se indica que se quiere acceder a la referencia en memoria de un array mutable
        *i = "PHP"; // el * indica que se accede al puntero de la variable, y se modifica su espacio en memoria
    }

    println!("{:?}", langs); // imprime PHP, PHP, PHP, PHP

}