#[allow(unused_variables)]
#[derive(Debug)]

struct Candidate {
    name: String,
    age: u8,
}

fn candidate_control(c: &Candidate) -> Result<String,String> {
    if c.age < 18 {
        Err("The candidate isn't proper!".to_string())
    } else {
        Ok("The candidate is proper.".to_string())
    }
}


fn main() {

    let candidate_1 = Candidate {name: String::from("Aybilge"), age: 17};

    let candidate_2 = Candidate {name: String::from("Hakan"), age: 34};
    
    println!("{:?}", candidate_control(&candidate_1));
    
    println!("{:?}", candidate_control(&candidate_2));

}


/*
Err("The candidate isn't proper!")
Ok("The candidate is proper.")
*/

