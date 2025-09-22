#![allow(unused)]

fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

fn max<T: std::cmp::PartialOrd>(s: &[T]) -> Option<&T> {
    if (s.len() == 0) {
        return None;
    }

    let mut largest = &s[0];

    for item in s {
        if item >= largest {
            largest = item
        }
    }

    Some(largest)
}

fn main() {
    let t: (u32, u32) = (1, 2);
    let s = swap(t);
    println!("{:?}", s);

    let t: (i32, u32) = (-1, 2);
    let s = swap(t);
    println!("{:?}", s);

    let nums = vec![33, 1, 22, 54, 25, 99, 10];
    let largest = max(&nums);
    println!("largest number: {:?}", largest);

    let chars = vec!['a', 'c', 'y', 'i', 'm'];
    let largest = max(&chars);
    println!("largest char: {:?}", largest);
}
