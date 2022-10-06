//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number {}.", secret_number);

    parsing();
    ordering();
}

fn parsing() {
    let mynumber = String::from("2342.33");

    let mynumber: f64 = mynumber.trim().parse().unwrap();

    println!("el número es {}", mynumber);

    let mynumber = String::from("2342.33");
    match mynumber.trim().parse::<i32>() {
        Ok(_) => panic!(),
        Err(_) => println!("cannot parse into i32 {}", &mynumber),
    };
}

fn ordering() {
    let one = 35;
    let two = 100;
    match one.cmp(&two) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => panic!(),
        Ordering::Equal => panic!(),
    }

    let one = "abcdefg";
    let two = "ABCDEFG";
    match one.cmp(&two) {
        Ordering::Less => panic!(),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => panic!(),
    }
}
