#[allow(unused_variables)]
#[derive(Debug)]

//We can use multiple data types
//Type1 and Type2 are my selections. You can use V, T, N etc... 

struct Info<Type1, Type2> {
    data_1: Type1,
    data_2: Type1,
    data_3: Type2, //Type2!
}

fn main() {

    let instance_1:Info<u8, i8> = Info{data_1: 12, data_2: 25, data_3: -10};
    
    println!("Number data: {:?}", instance_1);
    
    let instance_2:Info<String, bool> = Info{data_1: "Mustafa".to_string(), data_2: "Ulubey".to_string(), data_3: true};
    
    println!("Text data: {:?}", instance_2);

}


/*
Number data: Info { data_1: 12, data_2: 25, data_3: -10 }
Text data: Info { data_1: "Mustafa", data_2: "Ulubey", data_3: true }
*/

