// if式
// fn main() {
//    let number = 7;

  //   if number < 5 {
  //      println!("condition was true"); // 条件は真でした
  //   } else {
  //      println!("condition was false"); // 条件は偽でした
  //   }
// }

// 条件式はbool型である必要がある
// fn main() {
//    let number = 3;

  //   if number {
  //      println!("number was three"); // 数値は3です
  //   }
// }

// loopでコードを繰り返す
// fn main() {
//     loop {
//         println!("again!"); // また
//   }
// }

// breakで終了, continueでスキップ
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
              break 'counting_up;
            }
            remaining -= 1;
      }

      count += 1;
    }
    println!("End count = {}", count);
}