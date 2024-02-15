use std::io;

fn main() {

    let mut user_name = String::new();
    
    println!("Please, type your name: ");

    io::stdin().read_line(&mut user_name).expect("No user input!");
    
    println!("User name: {}", user_name); //User name: Mustafa
}

