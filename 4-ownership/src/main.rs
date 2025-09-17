// fromを使用してString型を生成
// fn main() {
//     let s = String::from("hello");
// }

// mutで可変することも可能
// fn main() {
//     let mut s = String::from("hello"); 
//     s.push_str(", world!"); // push_str関数でリテラル(", world"の部分)をStringに付け加える
//     println!("{}", s); // hello, world!と出力
// }

// xはスタック(i32のような小さい値)なので問題ない
// fn main() {
//     let x = 5;
//     let y = x;
//     println!("{}", x);
// }

// Stringはヒープ(大きい値)なので所有権がs2に移動(move)し、エラーが発生する
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// }

// // cloneを使うことでエラー解決(ただ、実行コストが高いため、パフォーマンスに注意)
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// スタックなので所有権のムーブが起きず、エラーが発生しない
// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }

// 所有権やスコープの例
// fn main() {
//     let s = String::from("hello"); // sがスコープに入る
//     takes_ownership(s); // sがtakes_ownership()に移動, String型なのでmoveする
//     let x = 5; // xがスコープに入る
//     makes_copy(x); // xがmake_copy()に移動, i32なのでエラーは発生しない
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // some_stringがスコープを抜けるとメモリが解放される

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// } // some_integerがスコープを抜ける


fn main() {
    let s1 = gives_ownership(); // 
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_String: String) -> String {
    a_String
}
