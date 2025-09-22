// fromを使用してString型を生成
fn main() {
    let s = String::from("hello");
}

// mutで可変することも可能
fn main() {
    let mut s = String::from("hello"); 
    s.push_str(", world!"); // push_str関数でリテラル(", world"の部分)をStringに付け加える
    println!("{}", s); // hello, world!と出力
}

// xはスタック(i32のような小さい値)なので問題ない
fn main() {
    let x = 5;
    let y = x;
    println!("{}", x);
}

// Stringはヒープ(大きい値)なので所有権がs2に移動(move)し、エラーが発生する
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
}

// cloneを使うことでエラー解決(ただ、実行コストが高いため、パフォーマンスに注意)
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// スタックなので所有権のムーブが起きず、エラーが発生しない
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

// 所有権やスコープの例
fn main() {
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s); // sがtakes_ownership()に移動, String型なのでmoveする
    let x = 5; // xがスコープに入る
    makes_copy(x); // xがmake_copy()に移動, i32なのでエラーは発生しない
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringがスコープを抜けるとメモリが解放される

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integerがスコープを抜ける

// 戻り値の所有権を移動する
fn main() {
    let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
    let s2 = String::from("hello"); // s2がスコープに入る
    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、何も起きない。s1もスコープを抜け、ドロップされる

fn gives_ownership() -> String {  // gives_ownershipは、戻り値を呼び出した関数にムーブする
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

// 複数の値を返せる
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}
