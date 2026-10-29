fn main() {
    // VARIABLES DE TIPO ENTERO
    let x: i32 = 7; // i32 es el tipo de variables
    let y: i32 = 2; // se usa para denotar entero con signo

    // VARIABLES DE TIPO FLOTANTE
    let a: f32 = 7.0;
    let b: f32 = 2.0;

    // OPERADORES ARITMÉTICOS
    let sum = x + y;    // operador de suma
    let rest = x - y;   // operador de resta
    let div = x / y;    // operador de división
    let div_f = a / b;
    let mul = x * y;    // operador de multiplicación
    let modl = x % y;   // operador de resto, retorna el resto de una división

    println!(
        "
        Suma de {x} + {y} = {sum}
        Resta de {x} - {y} = {rest}
        División de enteros {x} / {y} = {div}
        División de flotantes {a} / {b} = {div_f}
        Multiplicación de {x} / {y} = {mul}
        Resto de {x} % {y} = {modl}
        "
    );

    println!("===========================");

    // El resultado de la división es redondeado hacia abajo,
    // ya que se trata de dos valores enteros

    // OPERADORES RELACIONALES Y LÓGICOS
    let equal = x == y;         // comparación (es igual)
    let not_equal = x != y;     // comparación (es distinto)
    let is_more = x > y;        // es mayor
    let is_less = x < y;        // es menor
    let more_equal = x >= y;    // es mayor o igual
    let less_equal = x <= y;    // es menor o igual

    // Otros operadores lógicos
    // && Operador and (y)
    // || Operador or (o)
    // ! Operador not (no)

    println!(
        "
        ¿{x} es igual a {y}?: {equal}
        ¿{x} es diferent a {y}?: {not_equal}
        ¿{x} es mayor a {y}?: {is_more}
        ¿{x} es menor a {y}?: {is_less}
        ¿{x} es mayor o igual a {y}?: {more_equal}
        ¿{x} es menor o igual a {y}?: {less_equal}
        "
    );

    println!("===========================");
    
}
