//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

pub fn main() {
    let c = String::from("Esto");
    println!("Esto.len() {}", c.len());
    let c = String::from("áéíóú");
    println!("áéíóú.len() {}", c.len());
    println!("áéíóú slice  {}", &c[..2]);
    println!("áéíóú slice  {}", &c[..3]); // PANIC
}
