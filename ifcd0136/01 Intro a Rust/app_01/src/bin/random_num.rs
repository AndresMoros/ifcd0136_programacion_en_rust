use rand::RngExt;
fn main() {
    println!("Random number generator");
    let rand_num = rand::rng().random_range(1..=100);
    println!("The random number: {rand_num}");
}