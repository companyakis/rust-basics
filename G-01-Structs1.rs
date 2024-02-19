fn main() {

    let mustafa = Employee {employee_id: 1000, department_id: 101, title: "Founder".to_string()};
    
    println!("Employee ID: {}", mustafa.employee_id);
    println!("Department ID: {}", mustafa.department_id);
    println!("Title: {}", mustafa.title);
}

struct Employee {
    employee_id: u32,
    department_id: u8,
    title: String,
}

/*
Employee ID: 1000
Department ID: 101
Title: Founder
*/

