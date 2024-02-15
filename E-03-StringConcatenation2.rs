fn main() {

    let name = String::from("Mustafa");
    
    let surname = String::from(" Buyukdereli");
    
    let me = format!("{}{}", name, surname);
    
    println!("{}", me); //Mustafa Buyukdereli
    
    println!("{}", name); //not moved!

}
