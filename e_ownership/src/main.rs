fn main() {
    // 复制: 基本类型，由于存储在栈上，所有所有权是【直接复制】的
    let a = "1";
    let b = a;
    println!("原来的:{}, 之后的:{}", a, b);

    // 移交：引用类型的所有权移交，下面c将指向的内存块移交给d之后，c就已经失效了
    let c = String::from("emm");
    let d = c;
    println!("原来的c不能用了, 之后的:{}", d);

    // 克隆：上述第二种情况不满足使用的话，则需要使用clone
    let e = String::from("ok");
    let f = e.clone();
    println!("原来的:{}, 之后的:{}", e, f);

    // 函数参数的所有权机制
    let mut g = String::from("prefix");
    g = takes_ownership(g);
    println!("{}", g);

    // 引用的组件 borrow
    let h = String::from("emm");
}

fn takes_ownership(mut str: String) -> String {
    str = String::from(str.as_str());
    str.push_str("after");
    str
}