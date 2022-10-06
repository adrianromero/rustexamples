//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

#[derive(Debug)]
struct Pepe {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug)]
struct Jose {
    pub c: i32,
    pub d: i32,
}

impl Jose {
    fn consume(self) -> i32 {
        self.c + self.d
    }
}

impl From<Pepe> for Jose {
    fn from(t: Pepe) -> Self {
        Jose { c: t.a, d: t.b }
    }
}

pub fn main() {
    let pepe = Pepe { a: 4, b: 5 };
    let jose = Jose { c: 1, d: 2 };

    println!("pepe {}", do_things_with_jose(&pepe.into()));
    println!("pepe {}", do_things_with_jose(&jose));

    let antonio25 = Pepe { a: 25, b: 4 };
    let antonio2 = Pepe { a: 2, ..antonio25 };
    println!(
        "antonio25, antonio2 with new a: {:?}, {:?}",
        &antonio25, &antonio2
    );

    let d: i32 = jose.consume();

    println!("d {}", d);

    let mut j = 125;
    let mut myclosure = || {
        j = j * 2;
        println!("j is equal to {}", j);
    };

    myclosure();
    myclosure();

    println!("j is finally equal to {}", j);
    assert_eq!(j, 500);

    fn sumame(a: i32, b: i32) -> i32 {
        a + b
    }

    let myc = sumame;

    println!("c as sumame {}", myc(j, 1));

    let xs: [i32; 4] = [1, 3, 4, 4];

    println!("c as sumame {}", std::mem::size_of_val(&xs));

    // Manolo

    let m1 = Manolo {
        a: String::from("aaa"),
        b: String::from("bbb"),
    };
    do_destructure_n_consume_manolo(m1);
}

fn do_things_with_jose(value: &Jose) -> i32 {
    println!("Jose is ->{:?}", value);
    value.c + value.d
}

struct Manolo {
    a: String,
    b: String,
}

fn do_destructure_n_consume_manolo<T>(m: T)
where
    T: Into<Manolo>,
{
    let my_manolo = m.into();
    let Manolo { a, b } = my_manolo;

    println!("Maolo {}, {}", a, b);
}
