fn main() {

    let a_proverb = String::from("A barking dog never bites");

    tell_a_proverb(&a_proverb); //A barking dog never bites

    println!("{}", a_proverb); //A barking dog never bites
    
    tell_a_proverb(&a_proverb); //A barking dog never bites

}

fn tell_a_proverb(proverb: &String) {
    
    println!("{}", proverb);
}
