pub fn main() {
  let c = String::from("Esto");
  println!("Esto.len() {}", c.len());
  let c = String::from("áéíóú");
  println!("áéíóú {}", c.len());
  println!("áéíóú slice  {}", &c[..2]);
  println!("áéíóú slice  {}", &c[..3]); // PANIC
}
