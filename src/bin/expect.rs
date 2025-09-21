#![allow(unused)]

fn main() {
    // unwrap and expect
    // Option
    let x: Option<i32> = Some(3);

    let v = x.expect("x is none");
    println!("val = {v}");

    // Result
    let x = 1;
    let y = 1;
    let z: Result<i32, String> = Err("divide by 0".to_string());
    let v = z.unwrap();
    println!("val = {v}");
}
