// Rust 是一种 预编译静态类型（ahead-of-time compiled）语言
// main 函数是一个特殊的函数：在可执行的 Rust 程序中，它总是最先运行的代码
fn main() {
    // println! 调用了一个 Rust 宏（macro）。如果是调用函数，则应输入 println（没有!）
    println!("Hello world!");
}