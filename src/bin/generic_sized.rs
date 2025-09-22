#![allow(unused)]

// Sized and ?Sized

// Sized
// - Size is known at compile time
// - Automatically implemented for primitive types
// - Necessary for allocating values on the stack

// ?Sized
// - Size amy not be known at compile time
// - Examples = dynamically sized types, slices, trait objects

fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}

trait A {}

impl A for u32 {}

fn d(x: Box<dyn A>) {}

fn main() {
    // Examples
    // Primitive types
    let i: i32 = 1;
    let x: f64 = 1.0;
    let b: bool = true;

    f(i);
    f(x);
    f(b);

    struct S {
        i: i32,
        j: i32,
    }

    let s = S { i: 1, j: 1 };

    f(s);

    let arr: [i32; 4] = [0; 4];
    f(arr);
    f(&arr);

    // ?Sized
    let slice: &[i32] = &[1, 2, 3];
    g(slice);

    let s: &str = "hello";
    g(s);

    let v: Box<dyn A> = Box::new(1u32);
    g(&v);
}
