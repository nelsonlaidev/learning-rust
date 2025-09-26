#![allow(unused)]

// #[derive(Debug)]
// enum List<'a> {
//     Cons(i32, Box<&'a List<'a>>),
//     Nil,
// }

// Rc - reference count
// - Uses to share ownership for read only purposes
// - Keeps track of the number of references to the value wrapped in Rc
// - Reference count increases by 1 when Rc is cloned,
//   decreases by 1 when cloned Rc is dropped
// - Cloning a Rc never performs a deep copy
// - Single threaded use (multi threaded -> Arc)

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 3 <- 2 <- 1 <- Nil
    //      |
    // 4 <--+
    // let nil = Nil;
    // let one = Cons(1, Box::new(&nil));
    // let two = Cons(2, Box::new(&one));
    // let a = Cons(3, Box::new(&two));
    // let b = Cons(4, Box::new(&two));

    let list = Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))));
    println!("list: {}", Rc::strong_count(&list));
    let a = Cons(3, Rc::clone(&list));
    println!("3 <- list: {}", Rc::strong_count(&list));
    {
        let b = Cons(4, Rc::clone(&list));
        println!("4 <- list: {}", Rc::strong_count(&list));
    }
    println!("dropped: {}", Rc::strong_count(&list));

    let mut cur: &List = &a;
    // v: &i32
    // tail: &Rc<List>
    while let Cons(v, tail) = cur {
        print!("{} -> ", v);
        cur = tail;
    }
    println!("Nil");
}
