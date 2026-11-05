// Leer las calificaciones de un alumno en 3 asignaturas, 
// visualizar la nota media por pantalla

use std::io;
fn main() {
    println!("Introduce la nota dle alumno para obtener la media:");
    println!("---------------------------------------------------");

    println!("Nota de matemáticas:");
    let math = read_number_f64();
    
    println!("Nota de lengua:");
    let lang = read_number_f64();

    println!("Nota de ciencia:");
    let cience = read_number_f64();

    println!("---------------------------------------------------");
    let media = (math + cience + lang) / 3.0;
    println!("La nota media es: {:.2}", media);

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