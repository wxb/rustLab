use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 15.2
fn hello(name: &str) {
    println!("Hello, {}", name);
}

// 15.3
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(unused)]
fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello("Rust");
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 15.3
    {
        let c = CustomSmartPointer {
            data: String::from("c stuff"),
        };
    } // c 离开了作用域
    let d = CustomSmartPointer {
        data: "d stuff".into(),
    };
    let e = CustomSmartPointer {
        data: "e stuff".into(),
    };

    let f = CustomSmartPointer {
        data: "f stuff".into(),
    };
    // 因为不能禁用当值离开作用域时自动插入的 drop，并且不能显式调用 drop，如果我们需要强制提早清理值，可以使用 std::mem::drop 函数。
    // 可以通过传递希望提早强制丢弃的值作为参数。std::mem::drop 位于 prelude
    drop(f);

    println!("CustomSmartPointer created.");
} // d/e 离开了作用域, 实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 e 在 d 之前被丢弃
