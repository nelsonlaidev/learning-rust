#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    // HashSet
    let mut set: HashSet<u32> = HashSet::new();

    let inserted = set.insert(1);
    println!("inserted: {inserted}");

    let inserted = set.insert(1);
    println!("inserted: {inserted}");

    let contains: bool = set.contains(&1);
    println!("contains 1?: {contains}");

    let contains: bool = set.contains(&2);
    println!("contains 2?: {contains}");

    let contains: bool = set.contains(&3);
    println!("contains 3?: {contains}");
}
