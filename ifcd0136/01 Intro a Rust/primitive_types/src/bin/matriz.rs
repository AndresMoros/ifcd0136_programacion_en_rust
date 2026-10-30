fn main() {
    let mut matrix: [[i32;4];3] = [[0;4];3];

    for row in matrix {
        println!("{:?}", row);
    }

    matrix[1][2] = 99;

    println!("====================");

    for row in matrix {
        println!("{:?}", row);
    }
    
}