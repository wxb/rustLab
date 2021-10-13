fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);
    println!(
        "The area_tup of the rectangle is {} square pixels.",
        area_tup(rect)
    );

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle is {:?}", rectangle);
    println!("rectangle is {:#?}", rectangle);
    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rectangle)
    );

    let rectangle_1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle_1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(20);
    println!("sq area is {}", sq.area());

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    // 关联函数:允许在 impl 块中定义 不 以 self 作为参数的函数; 关联函数任然是函数而不是防范
    // 关联函数经常被用作返回一个结构体新实例的构造函数；
    // 使用结构体名和 :: 语法来调用这个关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
