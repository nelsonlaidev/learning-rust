#![allow(unused)]

// String and &str
fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (&[u8]) valid UTF-8

    // When to use String vs &str
    // String -> mutate or data needs to be owned
    // &str -> read only

    // String
    let msg: String = String::from("Hello Rust 🦀");
    let len: usize = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");

    // str - string slice
    // &str
    // - usually used str with reference (borrowed)
    // - immutable
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("slice: {s}");
    println!("len: {len}");

    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of the binary
    // - immutable because hard-coded inside binary
    let hello: &str = "Hello Rust";

    let s: &str = r#"
      { 
        "a": 1,
        "b": { "c": 2 },
        "d": 3
      }
    "#;
    println!("{s}");

    // Dereference coercion
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg;

    // Add &str to String
    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("{msg}");

    // Interpolation
    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    println!("{msg}");
}
