#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct methods
impl Point {
    // Associated functions - static methods
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // Methods
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let mut p = Point::zero();
    println!("{:?}", p);

    p.move_to(2.0, 1.0);
    println!("{:?}", p);

    let d = p.dist();
    println!("{:?}", d);
}
