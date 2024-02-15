fn main() {

    let a_proverb = String::from("A barking dog never bites");

    tell_a_proverb(a_proverb); //A barking dog never bites
    
    //error[E0382]: borrow of moved value: `a_proverb`
    //println!("{}", a_proverb); //value borrowed here after move
    
}

fn tell_a_proverb(proverb: String) {
    
    println!("{}", proverb);
}
