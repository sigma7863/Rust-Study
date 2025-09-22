use std::io;
fn main() {
    // コンパイルエラー(再代入できない)
    let x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // mutについて(不変から可変にする)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constについて(再代入できない)
    const MAX_POINTS: u32 = 100_000;

    // シャドーイング
    let x = 5;  
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x); // xの値は{}です
    }
    println!("The value of x is: {}", x);
}
