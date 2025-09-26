#![allow(unused)]

use core::panic;
use std::sync::{Mutex, MutexGuard};
use std::thread;
// Arc (Rc) + Mutex (RefCell)

fn main() {
    let m: Mutex<i32> = Mutex::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            let mut v: MutexGuard<'_, i32> = m.lock().unwrap();
            panic!("thread failed!!!");
            *v += 1;
        });

        scope.spawn(|| {
            let res = m.lock();
            println!("lock result: {:?}", res);
        });
    });

    println!("mutex: {:?}", m)
}
