// 常量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// 静态全局变量
static G1: i32 = 0; // 不可变
static mut G2: i32 = 1; // 可变

fn main() {
    // 变量
    demo_variables();

    // 常量
    demo_const();

    // 静态变量
    demo_static();

    // 变量遮蔽
    demo_shadow();

    let tup: (u32, f32, i8) = (500, 6.4, 1);
    let (_, b, _) = tup;
    println!("{}, {}", tup.0, b);

    // let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let arr = [1, 2, 3, 4, 5, 6];
    println!("arr is: {}", arr[0]);
}

fn demo_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}

fn demo_const() {
    println!("THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}

fn demo_shadow() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // mut 和遮蔽之间的另一个区别是：我们可以改变值的类型，
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

fn demo_static() {
    println!("The value of G1 is: {}", G1);

    // 可变静态变量（全局变量）无论读写都必须用 unsafe 修饰
    unsafe {
        println!("The value of G2 is: {}", G2);
        G2 = 2;
        println!("The value of G2 is: {}", G2);
    }

    // 全局变量的内存不是分配在当前函数栈上，so 函数退出时，并不会销毁全局变量占用的内存空间，程序退出才会回收
}
