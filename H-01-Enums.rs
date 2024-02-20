#[derive(Debug)]

enum FinancialCondition {Poor, Average, Super}

fn main() {

    let customer1_fin_con = FinancialCondition::Super;
    
    println!("He is a good customer and his financial condition is {:?}.", customer1_fin_con);
}


/*
He is a good customer and his financial condition is Super.
*/

