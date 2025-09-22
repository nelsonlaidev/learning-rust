#![allow(unused)]

// Static and dynamic dispatch

// Static dispatch
// - Function to call is known at compile time
// - Monomorphization (code size can be larger)

// Dynamic dispatch
// - Function to call is known at runtime
// - vtable lookup (code size can be smaller)
// - Runtime overhead

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self);
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dyn_dispatch(t: &dyn F) {
    t.f();
}

fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj = A;
    static_dispatch(&obj);
    let obj = B;
    static_dispatch(&obj);

    let input = "A";
    // Trait object
    // Value that implements a trait
    // but its concrete type is not known at compile time
    let obj: &dyn F = match input {
        "A" => &A,
        "B" => &B,
        _ => panic!(),
    };

    dyn_dispatch(obj);

    let obj: Box<dyn F> = match input {
        "A" => Box::new(A),
        "B" => Box::new(B),
        _ => panic!(),
    };

    dyn_dispatch_box(obj)
}
