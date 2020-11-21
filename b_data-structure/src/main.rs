fn main() {
    const TEST_CONST: u32 = 2;
    println!("const is {}", TEST_CONST);

    // float
    let b = 666.555;

    // bool
    let bol = true;

    // char
    let cha = 'a';

    // string
    let str = String::new();

    // tuple
    let tup: (i32, &str, bool) = (233, "emm", true);
    let (a, b, c) = tup;
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // array
    let arr: [i32; 3] = [1, 2, 2];
    let arr2 = ["emm"; 2]; // 相当于["emm", "emm"]
    let a = arr[6];
}

