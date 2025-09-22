#![allow(unused)]

use std::convert::{From, Into};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

// (u32, u32) -> Point
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

// Into
fn main() {
    let t: (u32, u32) = (1, 2);
    let p = Point::from(t);
    println!("{:?}", p);

    let p: Point = t.into();
    println!("{:?}", p);
}
