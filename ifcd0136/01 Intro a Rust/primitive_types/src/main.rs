use std::println;

#[allow(unused_variables)]
#[allow(unused_must_use)]

fn main() {
    // PRIMITIVOS ESCALARES

    println!("\n===============================");
    println!("PRIMITIVOS ESCALARES");
    println!("===============================\n");

    let character: char = '😀'; // dato de tipo caracter, solo pueden almacenar un caracter de texto en unicode

    let n1: i32 = -23; // tipo de dato númerico, entero con símbolo (signed)
    let n2: u32 = 23; // tipo de dato númerico, entero sin simbolo (unsigned)

    let nf: f64 = 3.14; // tipo de dato númerico flotante, con decimales

    let ok: bool = true; // tipo de datos booleano, solo puede ser true o false
    let not_ok: bool = n2 as i32 > n1;

    println!("Es n2: {n2} mayor que n1: {n1} \t {not_ok}");

    // PRIMITIVOS COMPUESTOS

    println!("\n===============================");
    println!("PRIMITIVOS COMPUESTOS");
    println!("===============================\n");

    println!("\n====================");
    println!("ARRAYS");
    println!("====================\n");

    // Array o arreglo
    // Tienen una longitud fija, y cada elemento debe ser del mismo tipo
    let dias = [
        "Lunes",
        "Martes",
        "Miercoles",
        "Jueves",
        "Viernes",
        "Sabado",
        "Domingo",
    ];
    // Si no se especifica el tipo de los elementos en la declaración
    // Rust infiere el tipo y longitud del array en tiempo de compilación

    // Para especificar los tipos y numero de elementos en el array se usa
    // [tipo_dato; n_elementos] como se ve a continuación:
    let precios: [i32; 5] = [19, 82, 12, 1, 90];

    println!("Array con valores declarados: {:#?}", precios);

    // let arr: [i32; 5];
    // Esto genera el siguiente error:
    // 33 |     let arr: [i32; 5];
    //    |         --- variable declarada aquí pero sin inicializar
    // 34 |
    // 35 |     println!("{:?}", arr);
    //    |                      ^^^ `arr` se usa aquí pero no está inicializada
    //    |
    //    = nota: este error se origina en la macro `$crate::format_args_nl`, que proviene de la expansión de la macro `println` (en compilaciones Nightly, ejecuta con -Z macro-backtrace para más información)
    // ayuda: considera asignarle un valor
    //    |
    // 33 |     let arr: [i32; 5] = [42; 5];
    //    |                       +++++++++

    // Se puede rellenar un array con [v_elemento; n_elementos]
    // donde v_elemento es el valor que tendrá cada elemento del array
    let arr: [i32; 5] = [0; 5];

    println!("Array con elementos inferidos: {:#?}", arr);

    // Se puede indicar un rango de elementos para imprimir
    println!("Mostrar un rango de elementos: {:#?}", &dias[0..=4]);

    // Imprimir un elemento concreto
    println!("El primer día de la semana es: {0}", dias[0]);

    let mut productos = ["Manzana", "Melón", "Durazno"];
    let precios = [10, 20, 31];

    println!("\nMétodos de los arrays");
    println!("====================\n");

    println!("Cantidad de frutas de frutas: {}", productos.len()); // devuelve la longitud del array
    println!("Primer precio: {:?}", precios.first()); // devuelve el primer elemento de un array, last() devuelve el ultimo
    println!("Frutas ordenadas: {:?}", productos.sort()); // ordena el array (debe ser mutable)
    
    println!("\n====================");
    println!("TUPLAS");
    println!("====================\n");

    // La tupla igual que el array tiene un tamaño fijo en memoria
    // sin embargo, puede almacenar varios elementos de diferente tipo

    let tup: (i32, &str, f64) = (23, "Rust", 3.14); // Se establece de manera manual los tipos de datos de cada posición

    let language = tup.1; // se utiliza el punto (.) para acceder al índice de una tupla

    let (x, y, z) = tup; // De esta manera se pueden extraer valores de un tupla

    println!("Tupla completa: {:?}", tup);
    println!("El valor de y es: {y}");
    println!("El valor del segundo elemento en la tabla es: {:?}", tup.1)
}
