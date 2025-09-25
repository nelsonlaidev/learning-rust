#![allow(unused)]

// Closure - anonymous function + capture variables in the environment

fn main() {
    let f = |x: u32, y: u32| -> u32 { x + y };

    let f = |x, y| x + y;
    let z = f(1, 2);
    println!("z = {}", z);

    // This will not compile
    // let z = f(1.0, 2.0);

    let f = |x, y| x + y;
    let z = f(1, 2);

    let v: i32 = 1;
    let f = |x: i32| x + v;

    let w = vec![1, 2, 3];
    let w2: Vec<i32> = w.iter().map(|x| x + 1).collect();
}
