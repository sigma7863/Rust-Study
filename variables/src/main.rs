use std::io;
fn main() {
    // ↓ コンパイルエラー(再代入できない)
    // let x = 5;
    // println!("The value of x is: {}", x);     // xの値は{}です
    // x = 6;
    // println!("The value of x is: {}", x);

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

    let guess: u32 = "42".parse().expect("Not a number!"); // 数字ではありません！

    //  浮動小数点型(float)
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数値演算
    // 足し算
    let sum = 5 + 10;

    // 引き算
    let difference = 99.5 - 4.3;

    // 掛け算
    let product = 4 * 30;

    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 0

    // 余り
    let remainder = 43 % 5;

    // 論理値型(bool)
    let t = true;
    let f: bool = false; // 明示的型注釈付き

    // 文字型(char, Unicode)
    let c = 'z'; // ※シングルクォーテーション
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; // ハート目の猫

    // 複合型(タプル型, 配列型)
    // タプル型
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 配列型(タプル型より厳しい型)
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0]; // 配列の要素にアクセス
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index."); // 配列の何番目の要素にアクセスするか指定してください
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); // 値の読み込みに失敗しました
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number"); // 入力された値は数字ではありません
    let element = a[index];
    println!(
        "The value of the element at index {} is {}", // {}番目の要素の値は{}です
        index, element
    );
}
