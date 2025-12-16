// "&"で借用する
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 借用したものを変更しようとするとエラーが起こる
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

// mutを加えることで可変な参照になり、エラーが発生しない
fn main() {
    let mut s = String::from("hello");

    change(&mut s); // 可変な参照を生成
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 変数などのデータは1つしか可変な参照を持てないため、エラー
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2 );
}

// これはスコープを抜けているのでエラーにならない
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let r2 = &mut s;
}

// 不変な参照をしている間は可変な参照をすることはできないため、エラー
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！
}

// &sで参照を返そうとしてダングリングポインタを生成してしまい、エラー
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// そのまま"s"を返すことでダングリングポインタを防ぐ
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
