fn main() {

    let name = "Mustafa".to_string();
    
    let name2 = &name;
    
    println!("My name is {}.", name); //My name is Mustafa. 

    println!("My name is {}.", name2); //My name is Mustafa.
}
