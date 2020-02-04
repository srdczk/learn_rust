const MAX_POINT: u32 = 100000;

fn main() {
    //1 .变量定义 let
    // 如果变量没有用 mut, 那么是不可变的
    let a = 1;
    let mut b: u32 = 1;
    println!("a = {}, b = {}", a, b);
    b = 2;
    println!("b = {}", b);
    // 2.隐藏性
    // 同名变量隐藏前面定义的变量
    let b: f32 = 1.1;
    println!("b = {}", b);
    // 3.常量
    println!("MAX_POINT = {}", MAX_POINT);
    println!("Hello, world!");
}
