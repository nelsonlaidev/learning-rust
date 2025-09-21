#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Initialize
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);

    // Get
    let val: Option<&u32> = scores.get("green");
    println!("val: {:?}", val);

    let val: Option<&u32> = scores.get("yellow");
    println!("val: {:?}", val);

    scores.insert("green".to_string(), 200);
    let val: Option<&u32> = scores.get("green");
    println!("val: {:?}", val);

    // Upsert
    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;

    let val: Option<&u32> = scores.get("blue");
    println!("blue val: {:?}", val);

    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;

    let val: Option<&u32> = scores.get("blue");
    println!("blue val: {:?}", val);
}
