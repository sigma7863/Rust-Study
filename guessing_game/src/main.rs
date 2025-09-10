use std::io;
use rand::Rng;
use std:: cmp::Ordering;

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    let serect_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", serect_number);
    
    loop {
        println!("Please input your guess."); // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!"); // 数値を入力してください！
        println!("You guessed: {}", guess); // 次のように予想しました: {}

        match guess.cmp(&serect_number) {
        Ordering::Less => println!("Small!"), // 小さすぎ！
        Ordering::Greater => println!("Big!"), // 大きすぎ！
        Ordering::Equal => {
            println!("You Win!"); // やったね！
            break;
            }
        }
    }
}
