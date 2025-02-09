use std::fmt;

#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

impl Developer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Java,
    Golang,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

fn main() {
    // 使用T::default()
    let dev1 = Developer::default();
    // 使用Default::default(), 此时类型无法从上下文推断，需要类型提示
    let dev2: Developer = Default::default();
    // 使用T::new()
    let dev3 = Developer::new("Tyr");
    println!("dev1:{}\ndev2:{}\ndev3:{}", dev1, dev2, dev3);
}
