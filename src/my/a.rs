#[derive(Debug)]
pub struct S {
    pub id: u32,
    pub name: String,
}

pub fn print() {
    println!("a");
}

use super::super::foo;

pub fn call_foo() {
    foo::print();
}
