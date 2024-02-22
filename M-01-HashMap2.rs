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
    
    //print an age info
    println!("Sylvester Jr is {:?} years old.", children["Sylvester Jr".into()]);
    
    //change age info
    children.insert("Sylvester Jr".into(), 2);
    
    println!("Sylvester Jr is {:?} years old.", children["Sylvester Jr".into()]);

}


/*
Sylvester Jr is 3 years old.
Sylvester Jr is 2 years old.
*/

