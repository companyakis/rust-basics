fn main() {

    println!("Perimeter 1: {}", perimeter_rectangular(10, 12)); //Perimeter 1: 44
    
    println!("Perimeter 2: {}", perimeter_rectangular(8, 17)); //Perimeter 2: 50

}

fn perimeter_rectangular(a: u16, b: u16) -> u16 {
    2 * (a + b)
}
