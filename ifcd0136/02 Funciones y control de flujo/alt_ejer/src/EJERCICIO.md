# Alternativas
1. Recibo por teclado 2 números y un operador que podrá ser (+, - , *, /) realizar la operación que corresponda y calcular el resultado.
2. Introducir un número y visualizar si es par o impar.
3. Introducir un número y visualizar si es positivo o negativo (0 es positivo).
4. Almacenar en una variable un número y pedir al usuario que escriba un número, si coincide con el introducido le indicamos acierto y si no, error.
5. Elaborar un programa que muestre el precio de un billete de autobús, que se calcula en base a los kilómetros de trayecto (0.30 Euros por Km), pero si el recorrido supera los 80 Km se aplica un 15 % de descuento y que si el trayecto se realiza en día laborable (‘L’) hay un 5 % de descuento respecto si es día festivo (‘F’). Datos que introducir: kilómetros de recorrido y tipo de día (L-F).
6. Dado el sueldo bruto de un individuo (pedir por teclado), calcular su pensión teniendo en cuenta que esta será el 80% del sueldo bruto, pero que como máximo podrá cobrar 1300 €
7. Realiza el siguiente menú de un cajero automático
```rust
8. use std::io;
9. fn main() {
10. let mut saldo = 1000.0;
11. loop {
12. //menú
13. println!("\n--- Opciones cajero ---");
14. println!("1. Consultar saldo");
15. println!("2. Ingresar");
16. println!("3. Retirar efectivo");
17. println!("4. Salir");
18. println!("Seleccionar una opcion: ");
19.
20. //leer opción
21. let mut op = String::new();
22. io::stdin().read_line(&mut op).expect("Error al leer");
23. let op = op.trim();
24.
25. //realizar operación según la opción elegida
26. match op {
27. "4" => {
28. println!("Hasta la próxima");
29. break;
30. }
31. _ => {
32. println!("Opción no válida");
33. }
34. }
35. } }
```