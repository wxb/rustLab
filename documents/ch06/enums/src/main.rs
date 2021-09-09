enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit")
            }
            Message::Move { a, b } => println!("Move"),
            Message::Write(write) => println!("Write"),
            Message::ChangeColor(color) => println!("ChangeColor"),
        }
        println!("abc")
    }
}

fn main() {
    let m: Message = Message::Write(String::from("enums"));
    m.call();

    let mm = Message::ChangeColor(1, 2, 3);
    mm.call();

    let mmm = Message::Quit;
    mmm.call();
}
