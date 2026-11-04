#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() {
    // 1. Declara una variable llamada edad e inicialízala con el valor 75.
    let age = 75;

    // 2. Elige el tipo adecuado para esta variable: let serie_kennedy: ?? = 112263;.
    let serie_kennedy: u32 = 112263;

    // 3. Almacena en una variable la duración media de un episodio de la serie Perdidos (42 minutos).
    let duration: u8 = 42;

    // 4. Declara las variables dia_nacimiento, mes_nacimiento y anio_nacimiento, elige los tipos
    // adecuados (en base al conocimiento que tienes hasta ahora) y almacena en ellas tu fecha de
    // nacimiento.
    let dia_nacimiento: u8 = 16;
    let mes_nacimiento: u8 = 7;
    let anio_nacimiento: u16 = 2003;

    // 5. En la serie Caleidoscopio, una banda de ladrones profesionales intenta forzar una caja fuerte
    // irrompible para hacerse con el mayor botín de la historia: siete mil millones de dólares.
    // Implementa una variable que almacene esta cantidad.
    let botin: u64 = 7_000_000_000;

    // 6. ¿Tienes el modo oscuro activado en tu navegador? Crea una variable booleana que almacene
    // tu respuesta (true si está activo, false si no lo está).
    let dark_mode: bool = true;

    // 7. La primera versión estable de Rust fue lanzada en el año 2015. Almacena este dato en una
    // constante.
    const REALEASE: u16 = 2015;

    // 8. Declara una variable llamada numero_de_puntos cuyo valor pueda cambiar a lo largo de la
    // ejecución de un programa.
    let mut n_puntos: i32 = 0;

    // 9. El siguiente programa no funciona. Copia el código en tu editor de código favorito y haz que
    // compile.

    // let edad: f64 = 30;
    // let altura: &f32 = 182.00;
    // let inicial_nombre = E;
    // edad = 32;
    // altura = 183;
    // modificador_caracter: &char = &inicial_nombre;

    let mut edad: u8 = 30;
    let altura: f32 = 182.00;
    let inicial_nombre = 'E';
    edad = 32;
    let altura = 183.00;
    let modificador_caracter: &char = &inicial_nombre;
}