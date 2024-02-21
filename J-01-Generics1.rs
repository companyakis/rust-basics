#[allow(unused_variables)]
#[derive(Debug)]

//Type1 is my selection. You can use V, T, N etc... 
struct Info<Type1> {
    data_1: Type1,
    data_2: Type1,
    data_3: Type1,
}

fn main() {

    let instance_1:Info<u8> = Info{data_1: 12, data_2: 25, data_3: 150};
    
    println!("Number data: {:?}", instance_1);
    
    let instance_2:Info<String> = Info{data_1: "Mustafa".to_string(), data_2: "Ulubey".to_string(), data_3: "Subutay".to_string()};
    
    println!("Text data: {:?}", instance_2);

}

/*
Number data: Info { data_1: 12, data_2: 25, data_3: 150 }
Text data: Info { data_1: "Mustafa", data_2: "Ulubey", data_3: "Subutay" }
*/

