#[allow(unused_variables)]

fn main() {

    let mut v2 = vec![100, 200, 300];
    
    println!("{:?}", v2[1]); //200
    
    v2.pop();
    
    println!("{:?}", v2); //[100, 200]
    
    v2.push(1111);
    
    println!("{:?}", v2); //[100, 200, 1111]

    v2.remove(0);
    
    println!("{:?}", v2); //[200, 1111]
    
    println!("{:?}", v2.len()); //2

}
