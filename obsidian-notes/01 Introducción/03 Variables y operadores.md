# Variables en Rust

# Inmutabilidad de variables
## Variables mutables
### Cambia el valor, no el tipo
## Redeclaración (shadowing)
Las variables pese a ser inmutables, pueden declararse nuevamente con el mismo nombre.

```rust
let precio: i32 = 100;
println!("{precio}"); // 100

let precio: i32 = 200;
println!("{precio}"); // 200
```

A diferecencia de la palabra clave **mut** sto NO reasigna el valor de la primera variables _precio_, sino que destruye la referencia en memoria de la variable, y crea una completamente nueva. 
Haciendo esto, se mantiene la inmutabildiad de la variable, y se incluso cambiar su tipo. Por ejemplo:

```rust
fn main() {
    let x = 5; // x es un entero (i32)
    let x = x + 1; // Primer shadowing: x vale 6
    
    let x = "hola"; // Segundo shadowing: x ahora es un String
    println!("El valor es: {}", x);
}
```
### Ámbito de la variable
Si el shadowing se hace dentro de un bloque { ... }, la variable nueva solo vive en ese ámbito; al salir del bloque, la variable original vuelve a estar accesible.

# Visualizar variables en salida de consola
## Interpolación de variables
## Expresiones en salida de datos
# Palabra clave `as` (parse)
# Operador ternario
En otros lenguajes, como JavaScript, existe el _ternary operator_: `?`, el cual evalua una expresión y ejecuta una tarea dependiendo de si es verdadero o falso:

```js
console.log("${true ? "Verdadero" : "Falso"}"); // Muestra 'Verdadero'
```
Este operador NO existe en Rust, pero puede simularse su sintaxis con una declaración `if {} else {}`.

```rust

let x = true;

println!("{}", if x {"Verdadero"} else {"Falso"}); // Imprime 'Verdadero'
```