fn main() {
    println!("---");
    demo_01();
    println!("---");
    demo_02(String::from("abc"), String::from("def"));
    println!("---");
    demo_03();
    println!("---");
    demo_04();
    println!("---");
    demo_05();
    println!("---");
    demo_06();
    println!("---");
    demo_rectangle();
    println!("---");
    demo_method();
    println!("---");
    demo_07();
    println!("---");
    demo_08();
    println!("---");
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn demo_01_a() -> User {
    // 隐式地返回实例
    User {
        username: String::from("虢国技酱"),
        email: String::from("虢国技酱@example.com"),
        active: true,
        sign_in_count: 1,
    }
}

fn demo_01() {
    let mut u = demo_01_a();
    println!("{:?}", u.active);
    println!("{:?}", u.sign_in_count);
    println!("{:?}", u.username);

    // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
    u.email = String::from("anotheremail@example.com");
    println!("{:?}", u.email);
}

fn demo_02(email: String, username: String) -> User {
    // 变量与字段同名时的字段初始化简写语法
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn demo_03() {
    demo_03_01(demo_01_a());
    demo_03_02(demo_01_a());
    demo_03_03(demo_01_a());

    // 变量与数据交互：Copy trait；移动；Clone trait
}

// 使用老结构体赋值给新结构图
fn demo_03_01(user1: User) {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("value@tt.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);
}

// 结构体更新语法: .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
fn demo_03_02(user1: User) {
    let user2 = User {
        email: String::from("value"),
        ..user1 // ..user1 必须放在最后
    };
    println!("{:?}", user2);

    // 因为username的赋值是String类型，赋值会造成 移动；移动后user1就不能用了
    // println!("{:?}", user1);
}

fn demo_03_03(user1: User) {
    // 如果使用user中的标量类型，那么user1还是可用的
    let user2 = User {
        email: String::from("value"),
        username: String::from("123"),
        ..user1
    };
    println!("{:?}", user2);

    // active 和 sign_in_count 的类型是实现 Copy trait 的类型，所以user1还可以用
    println!("{:?}", user1);
}

// 元组结构体
fn demo_04() {
    let tup: (u32, &str, f32) = (1, "abc", 0.9);
    println!("{:?}", tup);

    // 元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?},{:?}", black, origin);
}

// 类单元结构体
fn demo_05() {
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    println!("{:?}", subject);
}

// 结构体数据的所有权
fn demo_06() {
    // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期（lifetime）
    struct User<'a> {
        active: bool,
        username: &'a str,
        email: &'a str,
        sign_in_count: u64,
    }

    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    println!(
        "{}, {}, {}, {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );
}

// 计算长方形面积
fn demo_rectangle() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 1,
        height: 2,
    };
    println!("{:?} \n {:#?}", rect1, rect1);

    // 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏
    let scale = 2;
    let rect2 = Rectangle {
        // dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
        width: dbg!(30 * scale),
        height: 50,
    };
    //  我们不希望 dbg! 拥有 rect1 的所有权,所以这里调用 dbg! 时传入一个引用
    dbg!(&rect2);
}

// 定义方法
fn demo_method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 要在 self 前面使用 & 来表示这个方法借用了 Self 实例
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // &self 来替代 rectangle: &Rectangle
        fn area_01(self: &Rectangle) -> u32 {
            self.width * self.height
        }

        // &self 实际上是 self: &Self 的缩写
        fn area_02(self: &Self) -> u32 {
            self.width * self.height
        }

        // 方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
        fn area_03(self: Self) -> u32 {
            self.width * self.height
        }
        // 果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
        fn area_04(self: &mut Self) -> u32 {
            self.width = 100;
            self.width * self.height
        }

        // 可以选择将方法的名称与结构中的一个字段相同
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels. {} {}",
        rect1.area(),
        rect1.area_01(),
        rect1.area_02()
    )
}

// 自动引用和解引用
fn demo_07() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };

    // 它是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
    // 这些代码是等价的：
    p1.distance(&p2);
    (&p1).distance(&p2);

    // 这种自动引用的行为之所以有效，是因为方法有一个明确的接收者— self 的类型。
    // 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
    // 注意：golang里面也有这种【自动引用和解引用】的能力
}

// 关联函数
fn demo_08() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 所有在 impl 块中定义的、不以 self 为第一参数的函数被称为关联函数
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // 使用结构体名和 :: 语法来调用这个关联函数
    let sq = Rectangle::square(3);
    println!("{:?}", sq);

    // 每个结构体都允许拥有多个 impl 块
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect = Rectangle {
        width: 90,
        height: 10,
    };
    println!("{:?}", rect.can_hold(&sq));
}
