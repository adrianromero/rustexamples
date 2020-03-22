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
}

fn do_things_with_jose(value: &Jose) -> i32 {
    println!("Jose is ->{:?}", value);
    value.c + value.d
}
