fn main() {

    let rectangular1 = Rectangular {height: 12, width: 18};
    
    rectangular1.area();
    
    println!("Perimeter: {}", rectangular1.perimeter());

}

struct Rectangular {
  height: u32,
  width: u32,

}

impl Rectangular {
    
    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
    
    fn area (&self) {
        println!("Area: {}", self.height * self.width);
    }
}

/*
Area: 216
Perimeter: 60
*/

