#![allow(unused)]

// Multi producer, single consumer
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

// Channel
fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    // std::mem::drop(rx);

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            tx_clone.send(format!("hello {}", i)).unwrap();
        });
    }

    std::mem::drop(tx);

    loop {
        match rx.recv() {
            Ok(msg) => println!("res: {:#?}", msg),
            Err(err) => {
                println!("err: {:#?}", err);
                break;
            }
        }
    }
}
