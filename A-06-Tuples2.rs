fn main() {

    let my_info: (String, String, u8, bool) = ("Mustafa".to_string(), "Buyukdereli".to_string(), 99, true);
    
    println!("My info: {:?}", my_info); //My info: ("Mustafa", "Buyukdereli", 99, true)
    
    //variables
    
    let (name, surname, age, happiness) = my_info;
    
    println!("Name: {}", name); //Name: Mustafa

    println!("Age: {}", age); //Age: 99

}
