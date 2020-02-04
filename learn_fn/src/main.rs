/*
 * 注释
 * **/
fn other_fun() {
    println!("This is a function");
}
fn other_fun1(a: i32, b: u32) {
    println!("a = {}, b = {}", a, b);
}
fn other_fun2(a: i32, b: i32) -> i32 {
     a + b
}
fn main() {
    other_fun();
    other_fun1(-2, 3);
    println!("other_fun2(-2, -4) = {}", other_fun2(-2, -4));
    // 语句是执行一些操作, 但是不返回值的指令
    // 和 c++, java 不同 let x = (let y = 1) x
    // 表达式会计算一些值
    let y = {
        let x = 1;
        x + 1
    };
    println!("y = {}", y);
}
