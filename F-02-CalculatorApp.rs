use std::io;

fn main() {

    let mut number1 = String::new();
    let mut number2 = String::new();
    
    let mut selection = String::new();
    
    println!("Type number one: ");
    
    io::stdin().read_line(&mut number1).expect("Please, type number one again!..");
    
    let number1: f32 = number1.trim().parse().expect("Please, try again!");
    
    println!("Type number two: ");
    
    io::stdin().read_line(&mut number2).expect("Please, type number two again!..");
    
    let number2: f32 = number2.trim().parse().expect("Please, try again!");
    
    println!("Type 1 for number1 + number2");
    println!("Type 2 for number1 - number2");
    println!("Type 3 for number1 * number2");
    println!("Type 4 for number1 / number2");
    
    io::stdin().read_line(&mut selection).expect("Wrong selection!..");
    
    let selection: u8 = selection.trim().parse().expect("Please, try again!");
    
    match selection {
        1 => addition(&number1, &number2),
        2 => subtraction(&number1, &number2),
        3 => multiplication(&number1, &number2),
        4 => division(&number1, &number2),
        _ => println!("Unexpected result. Try again..."),
    }
    
    
}

fn addition(x: &f32, y: &f32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn subtraction(x: &f32, y: &f32) {
    println!("{} - {} = {}", x, y, x - y);
}

fn multiplication(x: &f32, y: &f32) {
    println!("{} * {} = {}", x, y, x * y);
}

fn division(x: &f32, y: &f32) {
    println!("{} / {} = {}", x, y, x / y);
}

/*
Type number one: 
37
Type number two: 
26
Type 1 for number1 + number2
Type 2 for number1 - number2
Type 3 for number1 * number2
Type 4 for number1 / number2
4
37 / 26 = 1.4230769
*/
