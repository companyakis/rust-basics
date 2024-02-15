fn main() {

    //string
    
    let mut person = String::from("Mustafa ");
    
    person.push_str("Buyukdereli");
    
    println!("{}", person); //Mustafa Buyukdereli
    
    let mut chars = "abcd".to_string();
    
    chars.push('e'); //single ''

    println!("{}", chars); //abcde
}
