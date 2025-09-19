// if式
fn main() {
   let number = 7;

    if number < 5 {
       println!("condition was true"); // 条件は真でした
    } else {
       println!("condition was false"); // 条件は偽でした
    }
}

// 条件式はbool型である必要がある
fn main() {
   let number = 3;

    if number {
       println!("number was three"); // 数値は3です
    }
}

// 条件式に論理値を与えればエラーが起きない
fn main() {
   let number = 3;

   if number != 0 {
     println!("number was something other than zero"); // 数値は0以外の何かです
   }
}

// else ifで複数の条件式を書ける
fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}

// letでif式を定義できる
fn main() {
   let condition = true;
   let number = if condition { 5 } else { 6 };

   println!("The value of number is: {}", number); // numberの値は、{}です
}

// 条件式の型がそれぞれ違うとエラーが起きる
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}

// loopでコードを繰り返す
fn main() {
    loop {
        println!("again!"); // また
  }
}

// breakで終了, continueでスキップ
fn main() {
    let mut count = 0;
    'counting_up: loop {   // 'を付けることによってラベルを定義してラベルを付けたところまで抜けるか設定できる
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

// whileでループする
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!"); // 発射！
}

// whileの条件式を間違えるとエラー + 毎回境界値チェックを行うため、遅くなる
fn main() {
   let a = [10, 20, 30, 40, 50];
   let mut index = 0;

   while index < 5 {
       println!("the value is: {}", a[index]); // 値は{}です

       index += 1;
   }
}

// ↑ forループを使えば解決！
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is; {}", element);
    }
}

// "rev"を使ってカウントダウン
fn main() {
    for number in(1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}