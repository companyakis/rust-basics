#[allow(unused_variables)]
#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

fn main() {

    let mut people = vec![Person{name: "Mustafa".to_string(), age: 18}, Person{name: "Aygün".to_string(), age: 18}];
    
    println!("{:?}", people);
    
    let bilge = Person {name: "Bilge".to_string(), age: 55};
    
    people.push(bilge);
    
    println!("{:?}", people);
    
    for p in people {
        
        println!("Name: {:?} and age: {:?}", p.name, p.age);
    }
}


/*
[Person { name: "Mustafa", age: 18 }, Person { name: "Aygün", age: 18 }]

[Person { name: "Mustafa", age: 18 }, Person { name: "Aygün", age: 18 }, Person { name: "Bilge", age: 55 }]

Name: "Mustafa" and age: 18
Name: "Aygün" and age: 18
Name: "Bilge" and age: 55
*/

