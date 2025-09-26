#![allow(unused)]

// Concurrency
// - Thread
// - Channel
// - Async / await programming

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let h1: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("h1: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let h2: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("h2: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();

    let v = vec![1u32, 2u32, 3u32];
    let h = thread::spawn(move || {
        println!("v: {:?}", v);
    });
    h.join().unwrap();

    let h = thread::spawn(|| {
        return 1u32;
    });

    match h.join() {
        Ok(val) => println!("val: {}", val),
        Err(err) => {}
    }
}
