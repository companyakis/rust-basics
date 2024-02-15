fn main() {

   say_hi("Mustafa");
   say_hi("Bilge");
   say_hi("Kültigin");

}

fn say_hi(name: &str) {
    println!("Hi {}!", name);
}

/*
Hi Mustafa!
Hi Bilge!
Hi Kültigin!

*/
