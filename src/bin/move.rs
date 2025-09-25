#![allow(unused)]

fn f(s: &mut String) {}

fn main() {
    let mut s = "🦀".to_string();

    f(&mut s);
    println!("{s}");

    // Closures can capture variables by
    // - Borrow immutable reference &T
    // - Borrow mutable reference &mut T
    // - Take ownership of value T

    // - Borrow immutable reference &T
    let s = "🦀".to_string();
    let f = || println!("borrow: {s}");
    f();
    println!("main: {s}");

    // - Borrow mutable reference &mut T
    let mut s = "🦀".to_string();
    let mut f = || s += " world";
    f();
    println!("main: {s}");

    // - Take ownership of value T
    let s = "🦀".to_string();
    let f = move || {
        println!("move: {s}");
        s
    };
    f();
    // f();
    // println!("main: {s}");
}
