fn main() {

    //A tuple is a grouping of values of different types collected into one compound value
    //The individual values in a tuple are called elements
    //A tuple has a fixed length, which is equal to its number of elements. 
    //After a tuple is declared, it can't grow or shrink in size. 
    
    let my_info_tuple = ("Mustafa", 'S', 99, true); //name, marital status (Single/Married), age and happiness
    
    //Access elements in a tuple
    
    let var_name = my_info_tuple.0;
    let var_marital_status = my_info_tuple.1;
    let var_age = my_info_tuple.2;
    let var_happiness = my_info_tuple.3;
    
    println!("{var_name}");
    println!("{var_marital_status}");
    println!("{var_age}");
    println!("{var_happiness}"); 
}
