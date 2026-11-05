// Leer dos números y almacenarlos en dos variables, (ejemplo: n1 -> 5, n2 -> 7). 
// Cambiar los valores. (en el ejemplo, n1 será 7 y n2 será 5)

use std::io;

fn main() {

    println!("Primer numero se almacena en x:");
    let mut x = read_number_f64();

    println!("Segundo numero se almacena en y:");
    let mut y = read_number_f64();

    println!("---------------------------------------------------");
    println!("x: {x} | y: {y}");

    let aux = x;

    println!("Se cambia el valor de y por el de x y viceversa:
    x = y;
    y = aux;");

    x = y;
    y = aux;

    println!("x: {x} | y: {y}");






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