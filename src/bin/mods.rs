#![allow(unused)]

// Modules
use learning_rust::my;

fn main() {
    my::print();
    my::a::print();

    let s = my::a::S {
        id: 1,
        name: "S".to_string(),
    };

    println!("{:?}", s);

    my::call_foo();
    my::a::call_foo();
}
