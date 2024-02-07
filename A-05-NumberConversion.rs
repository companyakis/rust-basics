fn main() {

    let num1 = 12u8;
    
    let num2 = 122u16;
    
    //let sum = num1 + num2;
    
    //println!("num1 + num2 = {}", sum); //error[E0277]: cannot add `u16` to `u8`

    let sum = num1 + num2 as u8;

    println!("num1 + num2 = {}", sum); //num1 + num2 = 134
    
}
