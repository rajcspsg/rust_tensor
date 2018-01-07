fn main() {
   let s = String::from("string");
   let y = &s;

   println!("s is {}", s);
   println!("y is {}", y);
   println!("s is {}", s);
}