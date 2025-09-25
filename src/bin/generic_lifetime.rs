#![allow(unused)]

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let x = "🦀".to_string();
    {
        let y = "😼😼".to_string();
        let z = longest_str(&x, &y);
        println!("z = {}", z);
        // y is dropped here
    }
    // x is dropped here
}
