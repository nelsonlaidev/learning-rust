#![allow(unused)]

use core::panic;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn main() {
    // Rc<RefCell>
    // Arc<Mutex>
    let m: Mutex<i32> = Mutex::new(0);
    let counter = Arc::new(m);

    let c1 = Arc::clone(&counter);
    let c2 = Arc::clone(&counter);

    let h1 = thread::spawn(move || {
        let mut v = c1.lock().unwrap();
        *v += 1;
    });

    let h2 = thread::spawn(move || {
        let mut v = c2.lock().unwrap();
        *v += 1;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("arc: {:?}", counter);

    // thread::scope(|scope| {
    //     scope.spawn(|| {});

    //     scope.spawn(|| {
    //         let res = m.lock();
    //         println!("lock result: {:?}", res);
    //     });
    // });

    // println!("mutex: {:?}", m)
}
