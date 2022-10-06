use std::fmt;

#[derive(Debug)]
struct Hola {
    a: i32,
    b: i32,
}

impl fmt::Display for Hola {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(a: {}, b: {})", self.a, self.b)
    }
}
fn unicode() {
    println!("Hello, world!");

    println!("latin {}", latin1_to_char(65));

    println!("char {:?}", char_to_latin1('A'));
    println!("char {:?}", char_to_latin1('Ñ'));
    println!("char {:?}", char_to_latin1('α'));
    println!("asbytes {:?}", "αAAÑ".as_bytes());
}

fn latin1_to_char(latin1: u8) -> char {
    latin1 as char
}
fn char_to_latin1(c: char) -> Option<u8> {
    if c as u32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}
fn main() {
    println!("Hello, world!");
    let h = Hola { a: 23, b: 333 };
    println!("La estructura es {}.", h);
    println!("La estructura en debug es {:?}.", h);
    unicode();
}
