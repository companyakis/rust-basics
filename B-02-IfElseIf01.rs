fn main() {

    let balance_ethereum: u8 = 140;
    
    if balance_ethereum >= 10 && balance_ethereum < 50 {
        println!("Good!");
    } else if balance_ethereum >= 50 && balance_ethereum < 100 {
        println!("Very good!");
    } else {
        println!("We should check and decide...");
    }
}
