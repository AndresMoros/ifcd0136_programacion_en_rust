#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {
    // MÉTODOS DE CADENAS DE TEXTO (tipo String)

    let mut s1 = String::new(); // vacío
    let s2 = String::from("hola");
    let s3 = "hola".to_string(); //pasa a String cualquier tipo
    let num = 42.to_string(); // número -> String
    let s4 = "hola".to_owned(); //pasa str a String , guarda en el head, ahora es mutable y dueño
    let s5 = String::with_capacity(50); // reserva capacidad conversión
    let num = 42.to_string(); // número -> String

    // MODIFICACIÓN DE CADENAS DE TEXTO (requiere mut)
    let mut s = String::from("Hola");
    s.push('!'); // añade un char
    s.push_str(" mundo"); // añade un &str
    s.insert(0, '¡'); // inserta char en posición (en bytes)
    // s.insert_str(1, "ey, "); // inserta &str
    // ERROR al insertar &str en byte 1 ocupado por ! que ocupa 2 bytes
    // ⚠️ Los índices son en bytes: paniquea si cortas en medio de un char UTF-8
    s.pop(); // quita y devuelve el último char (Option<char>)
    s.remove(0); // quita el char en un índice
    s.truncate(5); // corta a N bytes
    s.clear(); // vacía el string
    s.retain(|c| c != 'a'); // conserva solo los chars que cumplan

    // CONCATENACIÓN
    let a = String::from("Hola");
    let b = String::from(" mundo");
    let c = a + &b; // `a` se mueve, ya no se puede usar
    let d = format!("{}{}", c, "!"); // no mueve nada
    let mut e = String::new();
    e += "texto"; // equivale a push_str
    // CONSULTA
    let s = "Hola mundo";
    let string_length = s.len(); // longitud en BYTES (no chars)
    s.chars().count(); // número de caracteres
    let empty_string = if s.is_empty() { true } else { false };
    s.contains("mundo"); // bool
    s.starts_with("Hol");
    s.ends_with("do");
    s.find("mundo");
    s.rfind('o'); // desde el final
    s.matches("o").count(); // cuántas veces aparece

    // TRANSFORMACIÓN
    let s = " Hola Mundo ";
    let uppercase = s.to_uppercase(); // " HOLA MUNDO "
    let lowercase = s.to_lowercase(); // " hola mundo "
    let trimmed = s.trim(); // "Hola Mundo"
    let trimmed_s = s.trim_start();
    let trimmed_e = s.trim_end();
    let replaced_string = s.replace("Hola", "Adiós"); // reemplaza todas
    let replaced_n = s.replacen("o", "0", 1); // reemplaza solo N veces
    let repeated = s.repeat(3); // repite el string
    let x = "##hola".trim_start_matches('#');
    let x = "hola.rs".strip_suffix(".rs"); // Option<&str>
    let x = "v1.0".strip_prefix("v"); // Some("1.0")
}
