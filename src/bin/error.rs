#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivideByZero,
    Other,
}

fn divide(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivideByZero);
    }
    Ok(x / y)
}

fn main() {
    // Error
    // panic!("crash");

    // Option or Result
    let arr = [1, 2, 3];
    // arr[9];
    // Option<&i32> = Some(&i32) | None
    let x: Option<&i32> = arr.get(9);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("none"),
    }

    let x = 1;
    let y = 0;
    // Result<u32, MathError>
    let z = divide(x, y);
    match z {
        Ok(val) => println!("divide = {val}"),
        Err(err) => println!("err = {:?}", err),
    }
}
