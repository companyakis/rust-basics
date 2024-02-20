#[derive(Debug)]

enum FinancialCondition {Poor, Average, Super}

struct Customer {
    name: String,
    id: u16,
    fin_cond: FinancialCondition
}

fn main() {

    let customer_id_521 = Customer {
        name: "Hakan".to_string(),
        id: 521,
        fin_cond: FinancialCondition::Average
    };
    
    println!("Customer 521 summary => Name: {}, ID: {} and financial condition: {:?}", 
            customer_id_521.name,
            customer_id_521.id,
            customer_id_521.fin_cond
            );

}

/*
Customer 521 summary => Name: Hakan, ID: 521 and financial condition: Average

*/

