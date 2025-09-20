#![allow(unused)]

fn main() {
    let x: i32 = -123;
    // This will not compile
    // x += 1;

    let mut y: i32 = 123;
    y += 1;

    let z = -123;

    const NUM: u32 = 1;

    let x: i32 = -1;
    let x: bool = true;

    let v: Vec<_> = vec![1, 2, 3];
}
