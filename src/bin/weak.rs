#![allow(unused)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// Memory leak
// Rc<T> and Weak<T>
// Rc::strong_count and Rc::weak_count

// Memory leak
// - Allocated memory is no longer accessible but it is not freed
// - Variable is no longer accessible but it still exists in memory

// Weak<T>
// - Reference to data that may be live or de-allocated
// - Increments weak_count
// - Data can be dropped even if weak_count > 0

// How to use Weak<T>
// Rc::downgrade
// - Creates weak reference
// - weak_count increases by 1
// - Cannot access data behind reference
// Rc::upgrade
// - Upgrades weak to strong reference (Weak<T> -> Rc<T>)
// - strong_count increases
// - Can access data behind reference

#[derive(Debug)]
struct Node {
    val: u32,
    neighbors: RefCell<Vec<Weak<Node>>>,
}

fn main() {
    // Rc<T> - used to share ownership
    // Rc::new and Rc::clone +1 strong_count
    // Rc drop -1 strong_count
    // T is dropped when strong_count = 0
    let x = "rust".to_string();
    let r0 = Rc::new(x);
    println!("strong count: {}", Rc::strong_count(&r0));

    let w1: Weak<String> = Rc::downgrade(&r0);
    println!("--- w1 ---");
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));
    println!("w1: {:#?}", w1);

    let w2: Weak<String> = Rc::downgrade(&r0);
    println!("--- w2 ---");
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));

    let u = w1.upgrade();
    println!("--- upgrade ---");
    println!("upgrade w1: {:?}", u);
    println!("strong count: {}", Rc::strong_count(&r0));
    println!("weak count: {}", Rc::weak_count(&r0));

    // Drop all strong references
    std::mem::drop(u);
    std::mem::drop(r0);

    let u = w1.upgrade();
    println!("--- drop ---");
    println!("upgrade w1: {:?}", u);

    // Reference cycle
    // node 0 -> node 1
    // node 1 -> node 0
    let node0 = Rc::new(Node {
        val: 0,
        neighbors: RefCell::new(vec![]),
    });
    let node1 = Rc::new(Node {
        val: 1,
        neighbors: RefCell::new(vec![]),
    });

    {
        let mut r0 = node0.neighbors.borrow_mut();
        r0.push(Rc::downgrade(&node1));

        let mut r1 = node1.neighbors.borrow_mut();
        r1.push(Rc::downgrade(&node0));
    }

    // Infinite loop
    println!("{:#?}", node0);

    // Memory leak
    println!("node 0: {:?}", Rc::strong_count(&node0));
    println!("node 1: {:?}", Rc::strong_count(&node1));

    std::mem::drop(node1);

    println!("node 0: {:?}", Rc::strong_count(&node0));
}
