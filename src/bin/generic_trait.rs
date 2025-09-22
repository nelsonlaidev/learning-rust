#![allow(unused)]

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &u32 {
        &self.0
    }
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
    fn first(&self) -> &T {
        &self[0]
    }
}

impl<X, Y> List<(X, Y)> for [(X, Y); 2] {
    fn count(&self) -> usize {
        2
    }
    fn first(&self) -> &(X, Y) {
        &self[0]
    }
}

fn main() {
    let xy: (u32, u32) = (1, 2);
    println!("xy count: {:?}", xy.count());
    println!("xy first: {:?}", xy.first());

    let arr: [(u32, &str); 2] = [(1, "rust"), (2, "hello")];
    println!("arr count: {:?}", arr.count());
    println!("arr first: {:?}", arr.first());
}
