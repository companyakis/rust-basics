fn main() {
    
    let mut name = "Mustafa".to_string();
    
    name = "Kutluk".to_string();
    
    println!("My name is {}.", name); //My name is Kutluk.
    
    let name2 = &mut name;
    
    println!("My name is {}.", name2); //My name is Kutluk.
}
