fn main() {

    //Tuple struct with data types only
    //Assume target quarterly budget is 15000
    
    struct QuarterlyBudgets(u128, u128, u128, u128, bool);
    
    
    //Instantiate tuple structs, pass values in same order as types defined
    
    let quarterly_budgets_last_year = QuarterlyBudgets(12000, 17000, 23500, 18000, false);
    
    println!("Q1 result: {}", quarterly_budgets_last_year.0);
    println!("Q2 result: {}", quarterly_budgets_last_year.1);
    println!("Q3 result: {}", quarterly_budgets_last_year.2);
    println!("Q4 result: {}", quarterly_budgets_last_year.3);
    println!("Yearly Success?: {}", quarterly_budgets_last_year.4);
    
    
    /*
Q1 result: 12000
Q2 result: 17000
Q3 result: 23500
Q4 result: 18000
Yearly Success?: false
    */
    
    
}
