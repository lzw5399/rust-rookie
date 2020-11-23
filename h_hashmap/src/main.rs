use std::collections::HashMap;

fn main() {
    // 1. 声明
    let mut hm = HashMap::new();

    // 2. 插入
    hm.insert("a", 233);
    hm.insert("b", 66);
    hm.insert("c", 777);

    // 3. 判断是否存在
    if hm.contains_key("b") {
        println!("存在当前key，对应的值为:{}", hm["b"]);
    }

    // 4. 移除
    hm.remove("b");


    let filtered = hm
        .iter()
        .filter(|&(k,v)| *k == "c")
        .cloned()
        .collect();
}
