fn main() {
    let x = 5;
    
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x is: {}", x); // xの値は{}です
    }
    
    println!("The value of x is: {}", x);
}
