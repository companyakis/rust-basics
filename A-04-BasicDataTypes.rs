fn main() {

    //basic data types examples
    
    let num1: u8 = 12;
    
    let num2: u16 = 1133;
    
    let num3: u128 = 54684477;
    
    let num4: f32 = 11.11;
    
    let num5: f64 = 111.12222;
    
    println!("All the numbers: {num1}-{num2}-{num3}-{num4}-{num5}"); //All the numbers: 12-1133-54684477-11.11-111.12222
    
    let are_you_happy: bool = true;
    
    let is_he_here = false;
    
    let compare = 12 > 25; //false
    
    println!("Bools: {are_you_happy}, {is_he_here} and {compare}"); //Bools: true, false and false

    let first_letter: char = 'A';
    
    let smile = '😃'; //char
    
    println!("A letter and smile: {first_letter}-{smile}"); //A letter and smile: A-😃
    
    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    
    let my_name = "Mustafa";
    
    // Specify the data type "str" with the reference syntax "&str"
    
    let your_name: &str = "Unknown";
    
    println!("Our names are {my_name} and {your_name}."); //Our names are Mustafa and Unknown.

    //constant values

    const THIS_YEAR: i16 = 2024;
    
    println!("Constant value: {}", THIS_YEAR); //Constant value: 2024
    
}
