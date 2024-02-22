#[allow(unused_variables)]
#[derive(Debug)]

enum Employees {Aybilge, Aygun, Gokhan, Kultigin}

fn search_employee(name: &str) -> Result<Employees, String> {
    match name {
        "Aybilge" => Ok(Employees::Aybilge),
        "Aygun" => Ok(Employees::Aygun),
        "Gokhan" => Ok(Employees::Gokhan),
        "Kultigin" => Ok(Employees::Kultigin),
        _ => Err("Ghost employee:)".to_string()),
    }
}

fn print_employee(name: Employees) {
    println!("Selected employee: {:?}", name);
}


fn main() {

    let e1 = search_employee("John");
    
    match e1 {
        Ok(name) => print_employee(name),
        Err(e) => println!("Error: {}", e),
    }

}


/*
Error: Ghost employee:)
*/

