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
    
    //let's use for loops
    
    for name in children.keys() {
        println!("Name: {:?}", name);
    }
    
    for age in children.values() {
        println!("Age: {:?}", age);
    }
    
    for (name, age) in children.iter() {
        println!("Name: {:?} - age: {:?}", name, age);
    }
}


/*
Name: "Sylvester Jr"
Name: "Tweety"
Name: "Casper"
Name: "Sylvester"

Age: 3
Age: 3
Age: 752
Age: 7

Name: "Sylvester Jr" - age: 3
Name: "Tweety" - age: 3
Name: "Casper" - age: 752
Name: "Sylvester" - age: 7
*/

