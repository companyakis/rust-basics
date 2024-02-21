#[allow(unused_variables)]
#[derive(Debug)]

//We can use option type to control data fields
 
struct Employee {
    name: String,
    id: u16,
    age: Option<u8>,
    religion: Option<String>
}

fn main() {

    let employee_1 = Employee {
        name: String::from("Çağdaş"),
        id: 101,
        age: Some(11),
        religion: Some("Muslim".to_string()),
    };
    
    let employee_2 = Employee {
        name: String::from("Kağan"),
        id: 107,
        age: None,
        religion: None,
    };
    
    //let's control two data fields
    
    match employee_1.age {
        Some(d) => println!("Current age info: {}", d),
        None => println!("No data!")
    }
    
    match employee_2.religion {
        Some(d) => println!("Current religion info: {}", d),
        None => println!("No data!")
    }

}

/*
Current age info: 11
No data!
*/

