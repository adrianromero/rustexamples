//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use postgres::{Client, NoTls};

pub fn main() {
    let mut client = Client::connect(
        "host=localhost port=5432 dbname=hellodb user=tad password=tad",
        NoTls,
    )
    .unwrap();

    client
        .simple_query(
            r#"
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
              "#,
        )
        .unwrap();

    let name = "Ferris";
    let data = None::<&[u8]>;
    client
        .execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&name, &data],
        )
        .unwrap();

    for row in client
        .query("SELECT id, name, data FROM person", &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
}
