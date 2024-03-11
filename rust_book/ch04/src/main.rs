fn main() {
    println!("Hello, world!");

    demo_string();

    demo_move();
    demo_clone();

    demo_copy();

    demo_ownership();
    demo_returnvalue();
    demo_borrow();
}

// 内存与分配
fn demo_string() {
    // 堆上
    let mut s = String::from("hello");
    println!("{:?}", s);

    s.push_str(" world!");
    println!("{:?}", s);
}

// 变量与数据交互的方式（一）：移动
fn demo_move() {
    // s1 和 s2 的结构：ptr-len-cap
    let s1 = String::from("hello");
    let s2 = s1; // 移动

    // println!("{}", s1) // value borrowed here after move
    println!("{}", s2)

    // Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。
}

// 变量与数据交互的方式（二）：克隆
fn demo_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆

    println!("{} {}", s1, s2)
}

// 只在栈上的数据：拷贝
fn demo_copy() {
    // Rust 有一个叫做 Copy trait 的特殊标注，可以用在类似整型这样的存储在栈上的类型上
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
    let x = 5;
    let y = x;
    println!("{},{}", x, y);
    // 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。

    // 换句话说，这里没有深浅拷贝的区别，所以这里调用 clone 并不会与通常的浅拷贝有什么不同。
    let x = 5;
    let y = x.clone();
    println!("{},{}", x, y);
}

// 所有权与函数
fn demo_ownership() {
    // 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 返回值与作用域
fn demo_returnvalue() {
    let _s1 = gives_ownership(); // gives_ownership 将返回值
                                 // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let _s3 = takes_and_gives_back(s2); // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

// 引用与借用
fn demo_borrow() {
    // 将创建一个引用的行为称为 借用（borrowing）
    let s1 = String::from("hello world");
    let len = calculate_length(&s1);
    println!("{}", len);

    // 可变引用
    let mut s = String::from("hello");
    change(&mut s);

    // 可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用（解决数据竞争）
    {
        // 这里会报错：同时存在对s的可变引用r1和r2
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{} {}", r1, r2);
    }
    {
        // 这里不会报错：r1和r2不是同事存在对s的可变引用
        let r1 = &mut s;
        println!("{}", r1);
        let r2 = &mut s;
        println!("{}", r2);
    }

    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
}

fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
