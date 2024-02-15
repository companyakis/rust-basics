fn main() {

    //string and &str
    
    let mut person: &str = "Mustafa ";
    
    let a_truth = "Be honest!"; //&str
    
    //person.push_str("Buyukdereli"); //method not found in `&str`

    println!("{}", person); //Mustafa 
    
    person = "Bilge";
    
    println!("{}", person); //Bilge
    
    println!("{}", a_truth); //Be honest!
    
    //a_truth.push('!'); //method not found in `&str`

    
}
