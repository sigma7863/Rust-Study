// fn main() {
//   println!("Hello world!");

//   another_function();
// }

// fn another_function() {
//   println!("Another function."); // 別の関数
// }

// 関数に引数を持たせる
// fn main() {
//   another_function(5);
// }

// fn another_function(x: i32) {
//   println!("The value of x is: {}", x); // xの値は{}です
// }

// 複数の型を持たせる
// fn main() {
//   print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//   println!("The measurement is: {}{}", value, unit_label);
// }

// エラーが起こる(式指向言語のため)
// fn main() {
//     let x = (let y = 6);
// }

// 式(x + 1)にコロンは付けない
// fn main() {
//     let y = {let x = 3; x + 1};
//     println!("The value of y is: {}", y);
// }

// 関数の戻り値1
// fn five() -> i32 {5}

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }

// 関数の戻り値2
// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// 式(x + 1)に;(セミコロン)が付いているのでエラー
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}