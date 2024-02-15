fn main() {

    let name = String::from("Mustafa");
    
    let surname = String::from(" Buyukdereli");
    
    //let me = name + surname; //expected `&str`, found `String` 
    
    let me = name + &surname;
    
    println!("{}", me); //Mustafa Buyukdereli
    
    //println!("{}", name); //value borrowed here after move

}
