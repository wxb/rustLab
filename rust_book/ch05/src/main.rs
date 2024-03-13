fn main() {
    println!("Hello, world!");

    let mut u = demo_set();
    println!("{:?}", u.username);

    // 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
    u.email = String::from("anotheremail@example.com");
    println!("{:?}", u.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn demo_set() -> User {
    // 隐式地返回实例
    User {
        username: String::from("虢国技酱"),
        email: String::from("虢国技酱@example.com"),
        active: true,
        sign_in_count: 1,
    }
}
