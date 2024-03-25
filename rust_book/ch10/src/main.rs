fn main() {
    println!("Hello, world!");

    demo_01();
}

fn demo_01() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz123";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is: {}", result);
}

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
