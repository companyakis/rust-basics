fn main() {

  println!("Perimeter of rectangular: {}", perimeter_area_rectangular(12, 25).0);
  
  println!("Area of rectangular: {}", perimeter_area_rectangular(12, 25).1);

}

fn perimeter_area_rectangular(a: u16, b: u16) -> (u16, u16) {
    let perimeter = 2 * (a + b);
    let area = a * b;
    return (perimeter, area);
}
/*
Perimeter of rectangular: 74
Area of rectangular: 300
*/
