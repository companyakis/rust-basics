fn main() {

    // Classic struct with named fields
    
    struct Employee {name: String, id: u16, department: String}
    
    // Instantiate a struct
    
    let employee_1 = Employee {name: "Mustafa".to_string(), id: 1, department: "Future Innovations".to_string()};
    
    println!("Employee name: {}", employee_1.name);
    println!("Employee ID: {}", employee_1.id);
    println!("Employee department: {}", employee_1.department);
    
/*
Employee name: Mustafa
Employee ID: 1
Employee department: Future Innovations
*/
    
}
