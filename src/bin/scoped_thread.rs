#![allow(unused)]

use std::thread;

// Scoped thread
fn main() {
    let msg = "hello".to_string();

    let (v1, v2) = thread::scope(|scope| {
        let h1 = scope.spawn(|| {
            println!("msg: {}", msg);
            return 1u32;
        });
        let h2 = scope.spawn(|| {
            println!("msg2: {}", msg);
            return 2u32;
        });

        (h1.join().unwrap(), h2.join().unwrap())
    });

    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("msg: {}", msg);
}
