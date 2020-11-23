fn main() {
    // 1. 字符串的切片
    let str = String::from("abcdefg");
    let a = &str[0..4];
    // str.push_str("ee"); 被切片的字符串在borrow有效期内不允许改变
    println!("{}", a);

    // 2. 数组的切片
    let arr = [1, 2, 3, 4, 5, 6];
    let _slice_arr = &arr[..3];
}
