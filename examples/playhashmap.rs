//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use std::collections::HashMap;

pub fn main() {
    let mut pepe = HashMap::new();

    pepe.insert(String::from("one"), 1);
    pepe.entry(String::from("two")).or_insert(2);

    for (key, value) in &pepe {
        println!("{} -> {}", key, value);
    }

    for (key, value) in pepe.iter() {
        println!("{} -> {}", key, value);
    }

    println!("{:?}", pepe);
    for (key, value) in pepe {
        println!("{} -> {}", key, value);
    }

    let pepe = HashMap::from([(String::from("one"), 1)]);

    println!("{:?}", pepe);

    println!("{:?}", pepe.get("chorro"));
    println!("{:?}", pepe.get("one"));
}
