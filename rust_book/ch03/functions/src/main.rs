fn main() {
    println!("Hello, world!");

    another_function();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
