#![allow(unused)]

use crate::List::{Cons, Nil};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<RefCell<List>>),
    Nil,
}

fn main() {
    let mut s = "🦀".to_string();
    let s1 = &s;
    // s += "!";
    println!("{}", s1);

    let s = String::from("rust");
    let r: RefCell<String> = RefCell::new(s);
    {
        let mut r1: RefMut<'_, String> = r.borrow_mut();
        *r1 += "🦀";
        println!("{:#?}", r);

        // let mut r1: RefMut<'_, String> = r.borrow_mut();
        // *r1 += "🦀";
        // println!("{:#?}", r);
    }
    println!("{:#?}", r);

    // Interior mutability
    // - Allows data mutation even when there are
    //   immutable references to that data
    // - RefCell
    //   - Run time error when borrowing rules are broken
    //   - Single threaded use
    //   - RefCell is used in combination with Rc
    //     to create a mutable data with shared ownership
    // 2 -> 9 -> Nil
    //      |
    // 3 -> +
    let a = Rc::new(RefCell::new(Cons(1, Rc::new(RefCell::new(Nil)))));
    // 2 -> a
    let b = Cons(2, Rc::clone(&a));
    // 3 -> a
    let c = Cons(3, Rc::clone(&a));

    if let Cons(v, _) = &mut *a.borrow_mut() {
        *v = 9;
    }

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
