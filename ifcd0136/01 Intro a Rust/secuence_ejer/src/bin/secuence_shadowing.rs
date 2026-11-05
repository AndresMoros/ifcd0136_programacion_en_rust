// Estructuras del lenguaje – Secuenciales 
use std::io;

fn main() {

    // Se declaran las variables que almacenan los valores introducidos por el usuario

    let mut n: String = String::new(); // número que indicará la altura del rectángulo
    
    println!("Indica la altura del rectángulo:");
        io::stdin()
        .read_line(&mut n)
        .expect("Eso no es un número");

    // Se convierte la variables n de String a f64

    let height: f64 = n
    .replace(",", ".") // Remplaza el valor del primer parámetro por el del segundo en toda la cadena
    .trim() // Elimina los espacios al principio y al final de una cadena
    .parse() // Para que parse funcione, es necesario que se indique explícitamente el tipo de la variable
    .expect("n is NaN"); // Devuelve un mensaje de error si no fue posible la conversión

    // Se hace shadowing de la variable

    let mut n: String = String::new();

    println!("Indica la base del rectángulo:");
        io::stdin()
        .read_line(&mut n)
        .expect("Eso no es un número");

    let base: f64 = n
    .replace(",", ".")
    .trim()
    .parse()
    .expect("n is NaN");

    let perimetro = perimeter(&base, &height);
    let area = area(&base, &height);

    println!("La base es {base}, la altura es {height}");
    println!("El perimetro: {perimetro}, el área es {area}");

}

fn perimeter(base: &f64, height: &f64) -> f64 {
    (2.0 * base) + (2.0 * height)
}

fn area(base: &f64, height: &f64) -> f64 {
    base * height.round()
}

// 1. Pedir por teclado base y altura de un rectángulo y calcular el perímetro y el área. 
// • Perímetro=2 * base + 2 * altura 
// • Área=base * altura 
// 2. Leer las calificaciones de un alumno en 3 asignaturas, visualizar la nota media por pantalla 
// 3. Pedir dos números por teclado y visualizar el producto (n1 x n2) 
// 4. Leer dos números y almacenarlos en dos variables, (ejemplo: n1 -> 5, n2 -> 7). Cambiar los 
// valores. (en el ejemplo, n1 será 7 y n2 será 5) 
 
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