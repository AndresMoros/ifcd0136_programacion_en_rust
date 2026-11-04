// Estructuras del lenguaje – Secuenciales 
// 1. Pedir por teclado base y altura de un triángulo y calcular el perímetro y el área. 
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

use std::io;

fn main() {

    let mut entry: String = String::new();
    
        io::stdin()
        .read_line(&mut entry)
        .expect("Error");

    let num: i32 = entry.trim().parse().expect("NaN");

    
}
