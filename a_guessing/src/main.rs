// 默认会导入prelude(序曲)包下的
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数");

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("\n神秘数字是:{}", secret_number);

        println!("猜测一个数:");

        let mut guess = String::new();

        // io::stdin().read_line(&mut guess).expect("无法读取行");
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                println!("你猜测的数是：{}", guess.trim());
            }
            Err(err) => {
                println!("error:{}", err);
            }
        }

        // shadow: 允许在同一个作用域里面重复声明变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}

// 变量声明测试
// fn test() {
//     let mut foo = 1; // let foo = 1; 这种声明形式默认是immutable(不可变的)
//     foo = 2;
//
//     let str = "233";
//     println!("{} {}", foo, str);
// }

// expect测试
// fn pan() {
//     let x: Result<u32, &str> = Err("emergency failure");
//     x.expect("测试expect"); // panics with `Testing expect: emergency failure`
// }
