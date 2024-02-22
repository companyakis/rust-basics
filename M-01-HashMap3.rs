use std::collections::HashMap;

#[allow(unused_variables)]


fn main() {

    //keys and values
    //name and age pair

    let mut children = HashMap::new();
    
    children.insert("Casper", 752);
    children.insert("Sylvester", 7);
    children.insert("Tweety", 3);
    children.insert("Sylvester Jr", 3);
    
    //do we know Dalton Avarel age?
    
    match children.get("Avarel") {
        Some(age) => println!("Avarel is {:?} years old.", age),
        None => println!("No info"),
    }

}


/*
No info
*/

