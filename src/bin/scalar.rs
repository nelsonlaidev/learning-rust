#![allow(unused)]

fn main() {
    // Signed integers
    // -(2**(n - 1)) ~ 2**(n - 1) - 1
    let i0: i8 = 0; // - 2**(8 - 1) - 2**(8 - 1) - 1
    //                       - 128  - 127
    let i1: i16 = 1;
    let i1: i32 = 1;
    let i1: i64 = 1;
    let i1: i128 = 1;
    let i5: isize = 1;

    // Unsigned integers
    // 0 ~ 2**n - 1
    let u0: u8 = 1; // 0 ~ 2**8 - 1 = 255
    let u1: i16 = 1;
    let u2: i32 = 1;
    let u3: i64 = 1;
    let u4: i128 = 1;
    let u5: usize = 1;

    // Floats
    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';
    let e: char = '🦀';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and Max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("overflow u32: {u}");

    // checked_add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);

    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}
