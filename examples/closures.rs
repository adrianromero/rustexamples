//    Rust Examples is a collection of small portions of code written in Rust
//    Copyright (C) 2022 Adrián Romero Corchado.

fn main() {
    let f = my_function(AppInfo { two: 4 });
    println!("f(4)={}", f(4));
    println!("f(5)={}", f(5));
    println!("f(5)={}", f(5));
    println!("f(1)={}", f(1));

    let f2 = my_function2();
    println!("f2(4)={}", f2(4));
    println!("f2(5)={}", f2(5));
    println!("f2(5)={}", f2(5));
    println!("f2(1)={}", f2(1));

    let mut f3 = my_function3(AppInfo { two: 2 });
    println!("f3(4)={}", f3(4));
    println!("f3(5)={}", f3(5));
    println!("f3(5)={}", f3(5));
    println!("f3(1)={}", f3(1));
}

struct AppInfo {
    two: i32,
}

fn my_function(info: AppInfo) -> impl Fn(i32) -> i32 {
    return move |i: i32| -> i32 { info.two + i };
}

fn function_pointer(a: i32) -> i32 {
    a * a
}
fn my_function2() -> impl Fn(i32) -> i32 {
    return function_pointer;
}

fn my_function3(mut info: AppInfo) -> impl FnMut(i32) -> i32 {
    return move |i: i32| -> i32 {
        info.two = info.two + i;
        info.two
    };
}
