fn main() {

    //mutable tuple
    
    let mut my_info: (String, String, u8, bool) = ("Mustafa".to_string(), "Buyukdereli".to_string(), 99, true);
    
    println!("My age: {}", my_info.2); //My age: 99
    
    my_info.2 = 149;
    
    println!("My age: {}", my_info.2); //My age: 149
  
}
