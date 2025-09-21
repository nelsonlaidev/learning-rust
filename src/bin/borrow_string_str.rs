#![allow(unused)]

// String and str
fn take_string(s: String) {}
fn borrow_string(s: &String) {}
fn make_string() -> String {
    "".to_string()
}

fn mut_string(s: &mut String) {
    s.push_str("?");
}

fn borrow_str(s: &str) {}
// fn take_str(s: str) {}
// fn make_str() -> str {
//     ""
// }
// fn make_str() -> &str {
//     let s: &str = "";
//     s
// }

fn main() {
    // String
    // pub struct String {
    //     vec: Vec<u8>,
    // }
    // - Owned
    // - Mutable, growable
    // - Allocated on the heap
    // - &String can be coerced into &str
    let s = String::from("rust");
    take_string(s);
    // println!("{s}");

    // mut String
    let mut s = String::from("rust");
    s += "!";

    // &String
    let mut s = String::from("rust");
    borrow_string(&s);
    println!("{s}");
    borrow_str(&s);

    // &mut String
    let mut s = String::from("rust");
    let s1: &mut String = &mut s;
    mut_string(s1);
    println!("&mut String: {s}");

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size of the type not known at compile time
    // let a: str = "hello";
    // let b: str = "hello rust";

    // &str
    // - Size known at compile time (pointer)
    // - Immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("&str: {s}");

    // &mut str
    let mut s = String::from("hello");
    let r: &mut str = &mut s;
}
