fn main() {
    demo_u32();
    demo_f64();
    demo_math();
    demo_bool();
    demo_char();
    println!("以上就是标量类型");

    demo_tuple();
    demo_array();
    println!("以上就是符合类型");
}

// ------------------------- 👇🏻 标量类型 👇🏻 -------------------------
fn demo_u32() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The guess is: {}", guess);
}

fn demo_f64() {
    // 浮点数按照 IEEE-754 标准表示。f32 类型是单精度浮点型，f64 为双精度浮点型。
    // 默认浮点类型是 f64，因为在现代的 CPU 中它的速度与 f32 的几乎相同，但精度更高。
    // 所有浮点型都是有符号的。

    let x = 2.0;
    let y: f32 = 3.0;
    println!("The x is: {}; y is: {}", x, y)
}

fn demo_math() {
    // Rust提供的所有运算符：https://rustwiki.org/zh-CN/book/appendix-02-operators.html

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 整数除法会向下取整。

    // remainder
    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {} {}",
        sum, difference, product, quotient, floored, remainder
    )
}

fn demo_bool() {
    // 布尔值的大小为 1 个字节
    let t = true;
    let f: bool = false; // 显示类型注解

    println!("{} {}", t, f)
}

fn demo_char() {
    // Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值，这意味着它可以表示的远远不止是 ASCII。
    // char 字面量采用单引号括起来，这与字符串字面量不同，字符串字面量是用双引号括起来。

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat)
}

// ------------------------- 👆🏻 标量类型 👆🏻 -------------------------

// ------------------------- 👇🏻 复合类型 👇🏻 -------------------------

fn demo_tuple() {
    // 元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。元组的长度是固定的
    // 通过在小括号内写入以逗号分隔的值列表来创建一个元组。元组中的每个位置都有一个类型，并且元组中不同值的类型不要求是相同的。

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 模式匹配/模式解构
    let (x, y, _) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // 使用索引访问元组元素
    println!("The value of 3 is: {}", tup.2);
    // 单元类型
    // 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。该类型被称为单元类型，该值被称为单元值
    // 如果表达式不返回任何其他值，就隐式地返回单元值
}

fn demo_array() {
    // 数组的每个元素必须具有相同的类型。数组具有固定长度。

    // let a = [1, 2, 3, 4, 5];
}

// ------------------------- 👆🏻 复合类型 👆🏻 -------------------------
