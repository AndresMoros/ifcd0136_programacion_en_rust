fn main() {
    // Los elementos iteradores tienen métodos ->
    // Generadores:
    // .iter() | Recorre el elemento por referencia, conservamos el dato original
    // .iter_mut() | Recorre por referencia mutable y permite modificar los elementos
    // into_iter() | Mueve la propiedad de la conlección, el dato original se destruye

    let nums = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];

    let mut iterator = nums.iter();
    println!("{:?}", nums);
    // El método .next() devuelve el siguiente elemento de un iterador.
    // Esto consume el Item del iterador y este debe ser mutable
    println!("\niterator.next():");
    println!("{:?}", iterator.next());

    println!("println!(nums);");
    println!("{:?}", nums);
    println!("println!(nums[0]);");
    println!("{:?}", nums[0]);

    println!("iterator.next():");
    println!("{:?}", iterator.next());

    println!("Bucle while con .next():");

    while let Some(i) = iterator.next() {
        println!("{:?}", i)
    }
}
