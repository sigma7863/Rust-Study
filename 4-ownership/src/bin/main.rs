// 4_2-referencing-and-borrowing.rs
// "&"で借用する
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// 借用したものを変更しようとするとエラーが起こる
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// mutを加えることで可変な参照になり、エラーが発生しない
fn main() {
    let mut s = String::from("hello");

    change(&mut s); // 可変な参照を生成
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}