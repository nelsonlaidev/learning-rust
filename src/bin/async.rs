#![allow(unused)]

// Async / await
async fn g1() -> u32 {
    println!("g1");
    1
}

async fn g2() -> u32 {
    println!("g2");
    2
}

async fn g3() -> u32 {
    println!("g3");
    3
}

// Make coffee
async fn f() {
    println!("f");
    // Boil water - returns Future
    // let r1 = g1().await;
    // Grind coffee beans
    // let r2 = g2().await;

    // Boil water and grind coffee beans simultaneously
    let (r1, r2) = tokio::join!(g1(), g2());
    // Pour hot water on coffee grinds
    let r3 = g3().await;

    println!("{} {} {}", r1, r2, r3);
}

#[tokio::main]
async fn main() {
    f().await;
}
