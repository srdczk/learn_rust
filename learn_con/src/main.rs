fn main() {
    let y = 1;
    if y == 1 {
        println!("y = 1");
    }
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }
    if y == 0 {
        println!("y = 0");
    } else if y == 1 {
        println!("y = 1");
    } else if y == 2 {
        println!("y = 2");
    } else {
        println!("error");
    }
    // 在let中使用if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    println!("x = {}", x);
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }
        counter += 1;
    }
    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    println!("result = {}", result);
    let mut i = 1;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("ele = {}", element);
    }
    for ele in &arr {
        println!("ele = {}", ele);
    }
}
