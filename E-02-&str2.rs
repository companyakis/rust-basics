fn main() {

    //string and &str
    
    let person= String::from("Mustafa");
    
    let title: &str = &person[1..=4];
    
    let title2: &str = &person[1..4]; 
    
    println!("{}", title); //usta
    
    println!("{}", title2); //ust
    
}
