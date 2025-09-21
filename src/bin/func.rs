#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn print() {
    println!("no output");
}

// Diverge - never return
fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("crash")
}

fn main() {
    // Function
    // Implicit return
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    // No output
    print();

    // Diverge
    crash();
}
