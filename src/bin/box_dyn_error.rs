#![allow(unused)]

use std::{fs::File, io::Read, num::ParseIntError};

#[derive(Debug)]
enum MathError {
    DivideByZero,
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}

impl std::error::Error for MathError {}
impl std::error::Error for ParseError {}

fn f1() -> Result<u32, MathError> {
    Err(MathError::DivideByZero)
}

fn f2() -> Result<u32, ParseError> {
    Err(ParseError::InvalidInt)
}

fn f3() -> Result<(), Box<dyn std::error::Error>> {
    f1()?;
    f2()?;
    Ok(())
}

fn read(src_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut src_file = File::open(src_path)?;
    let mut data = String::new();
    src_file.read_to_string(&mut data)?;

    let lines: Vec<String> = data.trim().split("\n").map(|s| s.to_string()).collect();

    Ok(lines)
}

fn sum(lines: Vec<String>) -> Result<i32, ParseIntError> {
    let mut sum: i32 = 0;
    for line in lines {
        let num: i32 = line.parse()?;
        sum += num;
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let z = f3();
    println!("z = {:?}", z);

    let lines = read("./data/box_dyn_error.txt")?;
    let total = sum(lines)?;

    println!("total = {total}");

    Ok(())
}
