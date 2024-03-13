fn main() {
    println!("Hello, world!");

    let u = demo_set();
    println!("{:?}", u.username)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn demo_set() -> User {
    User {
        username: String::from("虢国技酱"),
        email: String::from("虢国技酱@example.com"),
        active: true,
        sign_in_count: 1,
    }
}
