use std::ops::Add;

fn main() {
    // 不可变字符串和可变字符串
    let str = "emm";
    let mut sb = String::from("dddd");
    sb = sb.add("okk");
    println!("Hello, world!, {}, {}", str.len(), sb);

    // local function
    fn plus_one(num: i32) -> i32 {
        num + 1
    }
    println!("调用local function = {}", plus_one(76));
}
