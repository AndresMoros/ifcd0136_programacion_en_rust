/*
Crear un array con 5 enteros
Recorrer el array
Guardar en una variable el máximo y en ora variable el mínimo
*/
fn main() {
    let numbers: [i32; 5] = [5, 1, 10, 3, 30];

    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];

    for &n in &numbers {
        if n > max {
            max = n;
        }
        if n < min {
            min = n;
        }
    }

    println!("Los valores son: {:#?}", numbers);
    println!("El valor más alto es: {max}");
    println!("El valor más bajo es: {min}");
}