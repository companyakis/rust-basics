fn main() {

//Default Parameters
//Rust doesn't support default parameters in functions
//We can use the Option type to simulate


give_info(None); //Dear patient, you will be fine!

give_info(Some("Sertaç")); //Dear Sertaç, you will be fine!

}

fn give_info(patient_name: Option<&str>) {

    match patient_name {
        Some(name) => println!("Dear {name}, you will be fine!"),
        None => println!("Dear patient, you will be fine!"),
    }
}

