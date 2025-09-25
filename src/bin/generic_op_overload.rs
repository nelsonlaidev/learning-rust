#![allow(unused)]

use std::ops::Add;

// Operator overload

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p0 = Point { x: 1.0, y: 2.0 };
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = p0 + p1;
    println!("{:?}", p2);
}
