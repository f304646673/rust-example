use std::io;

fn main() {
    println!("Hello, world!");
    // 变量默认是不可改变的（immutable）
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {guess}")
}
