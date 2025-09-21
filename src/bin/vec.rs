#![allow(unused)]

// Vector
fn main() {
    // Vec<T>
    let v: Vec<i32> = vec![-1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = Vec::new();

    let v = vec![1u8, 2, 3];
    let v = vec![1u8, 1, 1, 1, 1];
    let v = vec![1u8; 5];
    println!("v = {:?}, length = {}", v, v.len());

    // get
    let v = vec![1, 2, 3];
    let x = v[0];
    let x = v.get(99);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("invalid index"),
    }

    // update
    let mut v = vec![1, 2, 3];
    v[1] = 99;

    // push
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("push: {:?}", v);

    // pop
    let mut v: Vec<i32> = vec![1, 2, 3];
    match v.pop() {
        Some(val) => println!("pop: {val}"),
        None => println!("pop: v is none"),
    };
    match v.pop() {
        Some(val) => println!("pop: {val}"),
        None => println!("pop: v is none"),
    };
    match v.pop() {
        Some(val) => println!("pop: {val}"),
        None => println!("pop: v is none"),
    };
    match v.pop() {
        Some(val) => println!("pop: {val}"),
        None => println!("pop: v is none"),
    };

    // slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("slice: {:?}", s);
}
