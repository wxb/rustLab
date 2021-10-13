#![allow(unused)]

fn main() {
    // 新建Vec的两种方式：
    let mut v: Vec<i32> = Vec::new();
    println!("The v is {:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    println!("The v is {:?}", v);

    // 更新Vec方式
    v.push(10);
    println!("The v is {:?}", v);

    // 销毁Vec
    {
        let v = vec![1, 2, 3, 4];
        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃

    // 读取Vec项的两种方式
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 不能在相同作用域中同时存在可变和不可变引用的规则
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    v.push(6);
    // println!("The first element is: {}", first);

    // 遍历不可变 Vec
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // 遍历可变 Vec
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // vector 只能储存相同类型的值。
    // 这是很不方便的；幸运的是，枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
