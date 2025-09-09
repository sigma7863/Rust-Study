use std::io;

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    println!("Please input your guess."); // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // 行の読み込みに失敗しました

    println!("You guessed: {}", guess); // 次のように予想しました: {}
}
