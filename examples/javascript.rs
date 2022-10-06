//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

use boa::prelude::{JsResult, JsString, JsValue};
use boa::Context;

fn main() {
    let scenario = r#"

    function suma(a, b){
        return a + b;
    }

    const c =[3,3,3,2];

    print('This is a message. ');
    print('Suma javascript ', suma(4, 5));
    print(null);
    print(false);
    print(c);

    Math.PI ;
    "#;
    println!("{:?}", exec(scenario));
}

fn print(_: &JsValue, args: &[JsValue], c: &mut Context) -> JsResult<JsValue> {
    for element in args.iter() {
        print!("{}", element.to_string(c).unwrap_or(JsString::empty()));
    }
    println!();
    Ok(JsValue::Undefined)
}

fn exec<T: AsRef<[u8]>>(src: T) -> String {
    let src_bytes: &[u8] = src.as_ref();

    let mut context = Context::new();

    context.register_global_function("print", 1, print).unwrap();

    match context.eval(src_bytes) {
        Ok(value) => value.display().to_string(),
        Err(error) => error.display().to_string(),
    }
}
