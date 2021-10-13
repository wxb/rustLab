#![allow(unused)]

struct Bar;

impl Bar {
    fn some_method() -> i32 {
        20
    }

    fn echo() -> String {
        String::from("abc")
    }
}

trait BarTraits<T> {
    fn some_method(&self) -> T;
}

impl BarTraits<String> for Bar {
    fn some_method(&self) -> String {
        "Bar".into()
    }
}

fn main() {
    let a = Bar {};
    let c: i32 = Bar::some_method();
    println!("{}", a.some_method());
}
