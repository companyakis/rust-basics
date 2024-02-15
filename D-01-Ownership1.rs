fn main() {

    //we should understand what heap and stack are!
    
    let name = String::from("Mustafa");
    
    let name2 = name;
    
    //println!("My name is {}.", name); // error[E0382]: borrow of moved value: `name`

    println!("My name is {}.", name2); //My name is Mustafa.
}
