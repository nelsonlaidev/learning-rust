#![allow(unused)]

// Async vs thread
// native thread
// - good for CPU bound computation
// - limited by memory and OS thread limits
// Async programming
// - lower memory consumption
// - no limit on number of threads
// - good for I/O bound computations

use std::time::{Duration, Instant};
use tokio::join;

#[tokio::main]
async fn main() {
    // let mut handles = vec![];

    // for i in 0..100000 {
    //     let handle = std::thread::spawn(move || {
    //         std::thread::sleep(Duration::from_millis(100));
    //         println!("native thread: {}", i);
    //     });
    //     handles.push(handle);
    // }

    // for h in handles {
    //     h.join();
    // }

    let mut handles = vec![];

    for i in 0..1000000 {
        let fut = async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("async thread: {}", i);
        };

        let handle = tokio::task::spawn(fut);
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}
