fn main() {
    // bool 类型
    let is_true: bool = true;
    println!("is_true = {}", is_true);

    let is_false: bool = false; 
    println!("is_false = {}", is_false);
    // char 在rust 里面, char 是32 位的, 4个字节
    let a = 'a';
    println!("a = {}", a);
    let b = '你';
    println!("b = {}", b);
    // 数字类型
    // i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
    let c: i8 = -111;
    println!("c = {}", c);
    let d: f32 = 0.0;
    println!("d = {}", d);

    // 自适应类型 isize, usize
    println!("usize = {}", usize::max_value());
    // 数组
    // [Type; size] size 也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0] = {}", arr[0]);
    show(arr);
    // 元组, 3个元素
    let tup: (i32, f32, char) = (-2, 3.2, '你');
    println!("================");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("================");
    let (x, y, z) = tup;
    println!("{}={}={}", x, y, z);
}

fn show(arr:[u32; 5]) {
    println!("-----------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("------------------");
}
