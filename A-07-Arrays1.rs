fn main() {

    let array_1: [u8; 6] = [10, 20, 30, 35, 40, 200];
    
    let array_2 = [2023, 2024, 2025];
    
    let array_3: [&str; 4] = ["Kutluk", "Bumin", "Bilge", "Kultigin"];
    
    println!("Second element of array 2: {}", array_2[1]); //Second element of array 2: 2024
    
    //Use {:?}
    
    println!("Some Gokturk Names: {:?}", array_3); //Some Gokturk Names: ["Kutluk", "Bumin", "Bilge", "Kultigin"]
     

}
