use super::foo;

pub fn call_foo() {
    foo::print();
}

pub fn print() {
    f();
    println!("my");
}

fn f() {}

pub mod a;
