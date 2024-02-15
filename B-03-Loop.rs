fn main() {

    let mut counter: u8 = 0;
    
    loop {
        counter += 1;
        println!("Counter value: {}", counter);
        
        if counter == 15 {
            break;
        }
    }

}

/*
Counter value: 1
Counter value: 2
Counter value: 3
Counter value: 4
Counter value: 5
Counter value: 6
Counter value: 7
Counter value: 8
Counter value: 9
Counter value: 10
Counter value: 11
Counter value: 12
Counter value: 13
Counter value: 14
Counter value: 15
*/
