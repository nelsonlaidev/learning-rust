#![allow(unused)]

fn f<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}

// Trait bound - specifies constraints on a generic type
trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

fn c<T: A>(x: T) {}
fn m<T: A + B>(x: T) {}
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

// Difference between impl trait syntax and trait bounds
// x and y can be different types
fn k(x: impl A, y: impl A) {}
// x and y must be the same type
fn g<T: A>(x: T, y: T) {}
fn h<T: A, U: A>(x: T, y: U) {}

fn main() {
    let u: u32 = 1;
    let i: i32 = -1;
    let f: f32 = 1.0;

    c(u);
    c(i);
    // This does not compile
    // c(f);

    m(u);
    // This does not compile
    // m(i);

    w(u, u);
    // This does not compile
    // w(u, i);

    k(u, i);
    g(u, u);
    g(i, i);
    h(u, i);
    h(i, u);
}
