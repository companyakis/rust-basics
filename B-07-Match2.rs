fn main() {

    let langs_i_use: u8 = 4;
    
    let selection = match langs_i_use {
        
        1 => "Rust",
        2 => "Python",
        3 => "Dart",
        4 => "MQL5",
        5 => "Solidity",
        6 => "Javascript",
        7 => "Move",
        _ => "I don't use!",
    };
    
    println!("{} is important for algorithmic trading.", selection);

}

/*
MQL5 is important for algorithmic trading.

*/
