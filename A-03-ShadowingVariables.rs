fn main() {

    let monthly_budget = 10000;
    
    println!("Monthly budget: {monthly_budget}"); // Monthly budget: 10000
    
    //shadows existing variable
    let monthly_budget = 15000;
    
    println!("Monthly budget: {monthly_budget}"); // Monthly budget: 15000
    
    //shadows existing variable
    let monthly_budget = (monthly_budget * 2) + 2500;
    
    println!("Monthly budget: {monthly_budget}"); // Monthly budget: 32500
    
}
