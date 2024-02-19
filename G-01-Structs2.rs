fn main() {

    let calculation_1 = OurCalculations {x: 324, y: -12, z: 587};
    
    println!("Calculation result one: {}", calculation_1.add());
    
    println!("Calculation result two: {}", calculation_1.add_tolerance(-100));

}

struct OurCalculations {
  x: i32,
  y: i32,
  z: i32,
}

impl OurCalculations {
    
    fn add(&self) -> i32 {
        self.x + self.y + self.z
    }
    
    fn add_tolerance(&self, tolerance: i32) -> i32 {
        self.x + self.y + self.z + tolerance
    }
}

/*
Calculation result one: 899
Calculation result two: 799
*/

