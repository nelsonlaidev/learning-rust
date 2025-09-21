#![allow(unused)]

// Borrow and functions

fn take(s: String) {
    println!("take {s}")
}

fn borrow(s: &str) {
    println!("borrow {s}")
}

fn borrow_mut(s: &mut String) {
    s.push_str("🦀");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_ownership(s: String) -> String {
    println!("length = {}", s.len());
    s
}

fn print_len_borrow(s: &str) {
    println!("borrow length = {}", s.len());
}

fn main() {
    // Take ownership
    let s = String::from("rust");
    take(s);
    // println!("{s}");

    // Borrow immutable -> doesn't move ownership
    let s = String::from("rust");
    borrow(&s);
    println!("{s}");

    // Borrow mutable
    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");

    // Modify a function in 3 steps
    // 1. Take ownership
    let s = String::from("rust");
    print_len(s);

    // 2. Returns ownership
    let s = String::from("rust");
    let s = print_len_return_ownership(s);
    println!("{s}");

    // 3. Borrows
    let s = String::from("rust");
    print_len_borrow(&s);
    println!("{s}");
}
