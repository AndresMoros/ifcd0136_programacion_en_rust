fn main() {
    // VECTORES

    println!("DECLARACIÓN DE VECTORES CON MÉTODO ::new()");
    println!(
        " # Con ::new() se crea un vector vacío, y es necesario especificar el tipo:\nlet mut v: Vec<i32> = Vec::new();\n"
    );

    println!("DECLARACIÓN DE VECTORES CON MACRO");
    println!(
        " # Con `vec![]` se crea un vector con sus elementos y tipo ya inferidos:\nlet mut v = vec![1, 2, 3, 4, 5];\n"
    );

    // let mut v = vec![1, 2, 3, 4, 5];

    let mut v: Vec<i32> = Vec::new(); // Con ::new() se crea un vector vacío, y es necesario especificar el tipo

    println!("Vector vacío: {:?}\n", v);

    println!("Usamos la macro `vec![]` para meter elementos dentro:\nvec![1, 2, 3, 4, 5];");

    v = vec![1, 2, 3, 4, 5];

    println!(
        "Usamos el método `.push()` para meter elementos dentro:\nv.push(6);\nv.push(7);\nv.push(8);"
    );

    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}\n", v); // va a imprimir [1, 2, 3, 4, 5, 6, 7, 8]

    println!("Extraer el último elemento de un vector (modifica el vector): `v.pop();`");

    println!("Último elemento del vector: {:?}", v.pop());

    println!("{:?}\n", v); // va a imprimir [1, 2, 3, 4, 5, 6, 7]

    println!(
        "Insertar un elemento al vector (modifica el vector): `v.insert(i, e); (i: indice, e: elemento)`"
    );

    println!("Quiero insertar un 1000 en el indice 0 del vector: v.insert(0, 1000)");

    v.insert(0, 1000);

    println!("{:?}\n", v); // va a imprimir [0, 1, 2, 3, 4, 5, 6, 7]

    println!(
        "Extender al vector desde el final (modifica el vector): `v.extend([...]);`\nNota: La diferencia con `push()` es que con `extend()` puedo agreagar varios elementos"
    );

    println!(
        "Quiero agregar un 1001, 1002, 1003 al final del vector: v.extend([1001, 1002, 1003])"
    );

    v.extend([1001, 1002, 1003]);

    println!("{:?}\n", v); // va a imprimir [1000, 1, 2, 3, 4, 5, 6, 7, 1001, 1002, 1003]

    let mut x = vec![1004, 1005, 1006, 1007];

    println!(
        "Quiero unir mi vector `x [1001, 1002, 1003]` al final del vector v: v.apennd(&mut x)"
    );

    // Debe ser una referencia de otro vector mutable para asi modificar estos elementos nuevos

    v.append(&mut x);

    println!("{:?}\n", v); // va a imprimir [1000, 1, 2, 3, 4, 5, 6, 7, 1001, 1002, 1003, 1004, 1005, 1006, 1007]

    println!(
        "Quiero retener solo los elementos de mi vector que cumplan una condición: `v.retain(|&i| condición)\n&i es el elemento del vector con el que trabajamos;"
    );

    println!(
        "Quiero mantener todos los elementos que sean mayores de 1000: v.retain(|&i| i > 1000);"
    );

    v.retain(|&i| i > 1000);

    println!("{:?}\n", v); // va a imprimir [1001, 1002, 1003, 1004, 1005, 1006, 1007]

    // ACCESO A ELEMENTOS DE UN VECTOR

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    // Las originales se mueven al vector y ya no se pueden usar
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    // println!("{}", pepperoni); // Da error porque la propiedad de `pepperoni` fue movida al vector pizza_toppings

    // Tiene que ser con referencia
    //let pizza_slice = pizza_toppings[1]; //Error!
    // let pizza_slice = &pizza_toppings[1]; //Correcto
    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");
}
