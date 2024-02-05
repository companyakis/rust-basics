fn main() {
    
    //these variables are immutable!
    let this_year = 2024;
    
    let last_year;
    last_year = 2023;
    
    println!("This year: {} and last year: {}", this_year, last_year); //This year: 2024 and last year: 2023
    
    //use mut keyword
    
    let mut your_age = 29;
    println!("Your age is: {your_age}"); // Your age is: 29
    
    your_age = 18;
    println!("Your age is: {your_age}"); // Your age is: 18
    
}
