#![allow(unused)]

fn take(s: String) {}

fn copy(i: i32) {}

fn main() {
    // Memory - stack and heap
    // Stack
    // - Stores data of fixed size at compile time
    // - Fast
    // - LIFO
    // Heap
    // - Stores data of unknown size at compile time
    // - Slower than stack
    // - Data managed by ownership and borrowing rules

    // Ownership rules
    // 1. Each value has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // 1. Each value has an owner
    // Owner of "rust" is s
    let s = String::from("rust");
    // Owner of -1 is i
    let i: i32 = -1;

    // 2. There can only be one owner at a time
    let s = String::from("rust");
    // Owner of "rust" is s1
    let s1 = s;
    // println!("{s}");
    // Owner of "rust" is s2
    let s2 = s1;
    // println!("{s1}");
    println!("{s2}");

    // Owner of -1 is i
    let i: i32 = -1;
    // Owner of -1 is i1
    let i1 = i;
    println!("{i}, {i1}");

    // 3. When the owner goes out of scope, the value will be dropped
    let s = String::from("rust");

    {
        s;
    } // s is dropped
    // println!("{s}");

    let s = String::from("rust");

    {
        let s1 = s;
    } // s1 is dropped
    // println!("{s}");

    let s = String::from("rust");
    take(s);
    // println!("{s}");

    let i: i32 = -1;
    copy(i);
    println!("{i}")
}
