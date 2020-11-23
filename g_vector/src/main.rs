fn main() {
    // 1. vec的声明和push
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    for it in v.iter() {
        println!("{}", it);
    }

    // 2. filter
    let data = v
        .iter()
        .filter(|&x| *x > 2)
        .cloned()
        .collect::<Vec<i32>>();
    for it in data.iter() {
        println!("过滤后的:{}", it);
    }
}
