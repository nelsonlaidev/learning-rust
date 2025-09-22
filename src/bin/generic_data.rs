#![allow(unused)]

// Option, Result, Vec
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T = u32> {
    x: T,
    y: T,
}

fn main() {
    // Generic data types
    let x: Option<u32> = Option::Some(1);
    let x: Option<i32> = Option::Some(-1);

    let res: Result<bool, String> = Result::Ok(true);

    // Type will be inferred automatically
    let v: Vec<_> = vec![1, 2, 3];

    let p0 = Point { x: 0, y: 0 };
    let p1: Point<i32> = Point { x: -1, y: -1 };
}
