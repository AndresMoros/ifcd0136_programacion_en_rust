// Estructuras del lenguaje – Secuenciales
// 1. Pedir por teclado base y altura de un rectángulo y calcular el perímetro y el área.
// • Perímetro=2 * base + 2 * altura
// • Área=base * altura

use std::io;

fn main() {

    println!("Base del rectángulo");
    let base: f64 = read_number_f64();
    
    println!("Altura del rectángulo");
    let height: f64 = read_number_f64();

    let perimetro = perimeter(&base, &height);
    let area = area(&base, &height);

    println!("La base es {base}, la altura es {height}");
    println!("El perimetro: {perimetro}.");
    println!("El área: {area}.");
    
}

fn read_number_f64() -> f64 {
    // Se declaran las variables que almacenan los valores introducidos por el usuario
    let mut n: String = String::new(); 

    io::stdin().read_line(&mut n).expect("Eso no es un número");

    let n: f64 = n
        .replace(",", ".") // Remplaza el valor del primer parámetro por el del segundo en toda la cadena
        .trim() // Elimina los espacios al principio y al final de una cadena
        .parse() // Para que parse funcione, es necesario que se indique explícitamente el tipo de la variable
        .expect("n is NaN"); // Devuelve un mensaje de error si no fue posible la conversión

    n
}

fn perimeter(base: &f64, height: &f64) -> f64 {
    (2.0 * base) + (2.0 * height)
}

fn area(base: &f64, height: &f64) -> f64 {
    base * height.round()
}



// Leer desde teclado:
// Rust no tiene una función directa tipo scanf:
// Se lee una línea como String y luego se convierte (parsea) al tipo numérico.
// 1. Importar librería  → use std::io;
// 2. Se declara un String vacío → let mut entrada = String::new();
// 3. Leer línea completa desde teclado:
// io::stdin()
// .read_line(&mut entrada)   //añade texto al String, por eso es mutable
// .expect("Error al leer la línea");
// 4. Parsear a número:
// let numero: i32 = entrada.trim() //quita el salto de línea y los espacios
// .parse()
// .expect("Eso no es un número válido");  //si la entrada no es válida hace panic!. Más
// adelante veremos que se comprueban los datos con la estructura match.
// 5. Usar el número para lo que se necesite
// println!("El doble es {}", numero * 2);
