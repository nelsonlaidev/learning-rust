#![allow(unused)]

fn f_fn_pointer(f: fn(i32) -> i32) {}

// Fn, FnMut and FnOnce traits
// fn f_closure_input(f: ?) {}
fn f_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn f_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn f_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    // Fn (capture by &)
    // - Immutable borrow from environment
    // - Can be called more than once
    // FnMut (capture by &mut)
    // - Mutable borrow from environment
    // - Can be called more than once
    // FnOnce (capture by value)
    // - Moves captured values into closure, if needed
    // - Can be called at least once

    // Fn (capture by &)
    let s = "hello".to_string();
    let f = || println!("fn: {}", s);
    f_fn(f);
    println!("main: {}", s);

    // FnMut (capture by &mut)
    let mut v = vec![0];
    let mut f = || v.push(0);
    f_fn_mut(f);
    println!("main: {:?}", v);

    // FnOnce (capture by value)
    let v = vec![1, 2, 3];
    let f = move || println!("fn once: {:?}", v);
    f_fn_once(f);

    let v = 123;
    let f = move || println!("fn once: {:?}", v);
    f_fn_once(f);
    f_fn_once(f);
}
