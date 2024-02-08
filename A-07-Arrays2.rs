fn main() {

    //mutable arrays

    let mut array_1: [u8; 6] = [10, 20, 30, 35, 40, 200];
    
    println!("Our array: {:?}", array_1); //Our array: [10, 20, 30, 35, 40, 200]
    
    array_1[2] = 199;
    
    println!("Our array: {:?}", array_1); //Our array: [10, 20, 199, 35, 40, 200]
    
    //let's create 6*7 array
    
    let new_array = [6u8; 7];
    
    println!("Our new array: {:?}", new_array); //Our new array: [6, 6, 6, 6, 6, 6, 6]
}
