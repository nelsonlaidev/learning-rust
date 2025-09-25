#![allow(unused)]

// Function pointer
fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn do_twice(f: fn(u32, u32) -> u32, x: u32, y: u32) -> u32 {
    f(x, y) + f(x, y)
}

fn push(v: &mut Vec<u32>, x: u32) {
    v.push(x);
}

fn f_mut_twice(f: fn(&mut Vec<u32>, u32), v: &mut Vec<u32>, x: u32) {
    f(v, x);
    f(v, x);
}

fn main() {
    let f: fn(u32, u32) -> u32 = add;
    println!("f(1, 2) = {}", f(1, 2));

    let s = do_twice(add, 1, 2);
    println!("do twice = {}", s);

    let mut v = vec![1, 2, 3];
    f_mut_twice(push, &mut v, 4);
    println!("v = {:?}", v);
}
