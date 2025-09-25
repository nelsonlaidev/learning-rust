#![allow(unused)]

// Closure as function output
// Fn - captures variable by &T
fn f_fn() -> impl Fn(i32) -> i32 {
    let v: i32 = 1;
    move |x| x + v
}

fn f_fn_string() -> impl Fn() -> String {
    let s = "hello".to_string();
    move || {
        println!("s: {}", s);
        s.clone()
    }
}

// FnMut - captures variable by &mut T
fn f_fn_mut() -> impl FnMut() {
    let mut s = "hello".to_string();
    move || {
        s += "🦀";
        println!("fn mut: {}", s);
        // s.clone()
    }
}

// FnOnce - captures variable by T
fn f_fn_once() -> impl FnOnce() -> String {
    let s = "hello".to_string();
    move || {
        println!("fn once: {}", s);
        s
    }
}

fn main() {
    let f = f_fn();
    println!("f(1): {}", f(1));
    println!("f(1): {}", f(1));

    let mut f = f_fn_mut();
    f();
    f();

    let f = f_fn_once();
    let s: String = f();
    // let s: String = f();
}
