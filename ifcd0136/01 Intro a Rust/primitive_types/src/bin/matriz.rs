fn main() {
    let mut matrix: [[i32; 4]; 3] = [[0; 4]; 3];

    for row in matrix {
        println!("{:?}", row);
    }

    matrix[1][2] = 99;

    println!("====================\n");

    for row in matrix {
        println!("{:?}", row);
    }

    println!("====================\n");


    let matrix = 
    [
        ['X', 'O', 'X'],
        ['O', 'X', 'O'],
        ['O', 'O', 'X']
    ];

    let mut n_row = 0;

    for row in matrix {

        println!("Comienzo de la fila: {n_row}");

        let mut n_element = 0;

        for e in row {
            print!("El elemento {n_element} de la fila {n_row} es: {e}\n");
            n_element += 1;
        }
        println!("{:?}", row);
        println!("Fin de la fila: {n_row}\n");

        n_row += 1;
    }

}
