## Pregunta 1. ¿Cómo puedo corregir el siguiente código?

```rust
fn main() {
    let city = "Metropolis";
    city = "Gotham";
}
```

**Opciones de respuesta:**

- a. El código no necesita ninguna corrección. Compilará y se ejecutará correctamente.
- b. Agregue la palabra clave `mut` antes de la palabra clave `let` en la declaración de la variable.
- c. Agregue la palabra clave `mut` antes de la variable `city` cuando le asigne el nuevo valor `"Gotham"`.
- d. Agregue la palabra clave `mut` después de la palabra clave `let` en la declaración de la variable.

**Respuesta correcta:** d. Agregue la palabra clave `mut` después de la palabra clave `let` en la declaración de la variable.

---

## Pregunta 2. ¿Cuál es la sintaxis correcta para aplicar una directiva del compilador a la siguiente línea?

**Opciones de respuesta:**

- a. `#[allow(unused_variables)]`
- b. `[allow(unused_variables)]`
- c. `#[!allow(unused_variables)]`
- d. `!#[allow(unused_variables)]`
- e. `#(allow[unused_variables])`

**Respuesta correcta:** a. `#[allow(unused_variables)]`

---

## Pregunta 3. ¿Cuál será el resultado de este código?

```rust
fn main() {
    let latte = "Latte";
    let cappuccino = "Cappuccino";
    println!("I like {1} and {2}", latte, cappuccino);
}
```

**Opciones de respuesta:**

- a. `"I like latte and cappuccino"`
- b. El código no compila.
- c. `"I like Latte and Cappuccino"`
- d. `"I like Cappuccino and Latte"`

**Respuesta correcta:** b. El código no compila.

---

## Pregunta 4. ¿Qué carácter le indica al compilador que ignore una variable no utilizada?

**Opciones de respuesta:**

- a. `_`
- b. `?`
- c. `#`

**Respuesta correcta:** a. `_`

---

## Pregunta 5. ¿Qué caracteres forman un bloque?

**Opciones de respuesta:**

- a. Dos barras diagonales `//`.
- b. Un par de corchetes `[]`.
- c. Un par de paréntesis `()`.
- d. Apertura y cierre de llaves `{}`.

**Respuesta correcta:** d. Apertura y cierre de llaves `{}`.

---

## Pregunta 6. ¿Qué caracteres interpolan un valor dinámico en una cadena?

**Opciones de respuesta:**

- a. `<>`
- b. `:?`
- c. `""`
- d. `{}`

**Respuesta correcta:** d. `{}`

---

## Pregunta 7. ¿Qué palabra clave declara un alias o apodo para un tipo existente?

**Opciones de respuesta:**

- a. `rename`
- b. `alias`
- c. `redeclare`
- d. `type`

**Respuesta correcta:** d. `type`

---

## Pregunta 8. ¿Qué palabra clave declara una constante en el programa?

**Opciones de respuesta:**

- a. `final`
- b. `const`
- c. `let`
- d. `var`

**Respuesta correcta:** b. `const`

---

## Pregunta 9. ¿Qué palabra clave declara una variable como mutable?

**Opciones de respuesta:**

- a. `mut`
- b. `const`
- c. `mute`
- d. `mutable`

**Respuesta correcta:** a. `mut`

---

## Pregunta 10. ¿Qué palabra clave declara una variable?

**Opciones de respuesta:**

- a. `let`
- b. `const`
- c. `var`
- d. `declare`

**Respuesta correcta:** a. `let`

---

## Pregunta 11. ¿Qué palabra significa «la región del código donde un nombre es válido»?

**Opciones de respuesta:**

- a. Compilation
- b. Function
- c. Source code
- d. Scope

**Respuesta correcta:** d. Scope.

---

## Pregunta 12. ¿Qué resultado se obtendrá al ejecutar este código?

```rust
fn main() {
    let city = "Huntington";

    println!("{city}");

    let city = "Plainview";
}
```

**Opciones de respuesta:**

- a. `city`
- b. `Plainview`
- c. `{city}`
- d. `Huntington`

**Respuesta correcta:** d. `Huntington`

---

## Pregunta 13. ¿Qué tiene de malo el siguiente código?

```rust
fn main() {
    let mut genre = "Romance";
    genre = 52;
}
```

**Opciones de respuesta:**

- a. El código no tiene ningún problema.
- b. No debería haber un punto y coma después de la asignación de `52`.
- c. La palabra clave `mut` no es necesaria.
- d. El código está intentando asignar un número entero a una variable que espera una cadena.

**Respuesta correcta:** d. El código está intentando asignar un número entero a una variable que espera una cadena.

---

## Pregunta 14. ¿Una variable es ____ por defecto?

**Opciones de respuesta:**

- a. Mutable
- b. Inmutable

**Respuesta correcta:** b. Inmutable.

---

## Pregunta 15. Rust cuenta las posiciones a partir de:

**Opciones de respuesta:**

- a. `-1`
- b. `2`
- c. `0`
- d. `1`

**Respuesta correcta:** c. `0`