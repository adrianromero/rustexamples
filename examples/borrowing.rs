//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2020 Adrián Romero Corchado.
//
//    This file is part of Rust Examples
//
//     Licensed under the Apache License, Version 2.0 (the "License");
//     you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//         http://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

fn main() {

    let mut s : String = String::from("Initial String");
    s.push_str("\nPushed by owner");
    append_string(&mut s);
    println!("s (owner) -> {}", s) ;
   
    let sref : &mut String = &mut s;
    sref.push_str("\nPushed by mutable reference");
    println!("sref (&mut) -> {}", sref);

    append_string(sref);
    append_string(sref);
    
    s.push_str("\nPushed by owner");

    let sref : &mut String = &mut s; // borrow again because of previous one is borrowed again
    
    // s.push_str("\nTrying to push by owner");
    // ^ second mutable borrow occurs here

    sref.push_str("\nPushed by mutable reference");
    println!("sref (&mut) -> {}", sref);

    s.push_str("\nPushed by owner");

    // println!("sref (&mut) -> {}", sref);
    // ---- first borrow later used here

    println!("s (owner) -> {}", s) ;
}

pub fn append_string(pepe: &mut String) {
    pepe.push_str("\nPushed by function that borrows a mutable reference");
}
