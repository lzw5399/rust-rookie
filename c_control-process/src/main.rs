use std::ops::Range;

fn main() {
    // 方法的调用
    let mut s = sum(233, 555);
    println!("{}", s);

    // 表达式块的结果
    let y = {
        233
    };

    // 根据表达式块实现的三元表达式
    let a = if y == 233 { "是233" } else { "不是233" };

    // loop循环
    let result = loop {
        s += 1;
        if s == 1000 {
            break s;
        } else {
            break 666;
        }
    };
    println!("{}", result);

    // while循环
    while s < 0 {
        s -= 1;
    }

    // for循环
    let arr = [1, 2, 3, 4, 5, 6, 7];
    for v in arr.iter() {
        println!("{}", v);
    }

    for v in (100..105).rev() {
        println!("{}", v);
    }
}

/// # emm
/// - 1hao
/// - 2hao
fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}