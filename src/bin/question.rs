#![allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

fn f3() -> Result<u32, bool> {
    println!("f3");
    Ok(3)
}

fn f_match() -> Result<u32, String> {
    let res_1 = f1();
    let x1 = match res_1 {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    let res_2 = f2();
    let x2 = match res_2 {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    Ok(x1 + x2)
}

fn f_question() -> Result<u32, String> {
    let x1 = f1()?;
    let x2 = f2()?;
    let res_3 = f3();
    let x3 = match res_3 {
        Ok(x) => x,
        Err(err) => return Err("f3 error".to_string()),
    };
    Ok(x1 + x2)
}

// Question operator
fn main() -> Result<(), String> {
    // let res = f1();

    let z = f_question();
    match z {
        Ok(x) => println!("z = {x}"),
        Err(err) => println!("err {err}"),
    }

    let x = f1()?;
    println!("x = {x}");

    Ok(())
}
