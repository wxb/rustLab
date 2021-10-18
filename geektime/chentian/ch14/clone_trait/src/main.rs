#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Java,
    Golang,
}

fn main() {
    let dev = Developer {
        name: "Tyr".to_string(),
        age: 18,
        lang: Language::Rust,
    };

    let dev_1 = dev.clone();
    println!("dev:{:?}, addr of dev name:{:p}", dev, dev.name.as_str());
    println!(
        "dev_1:{:?}, addr of dev_1 name:{:p}",
        dev_1,
        dev_1.name.as_str()
    );
}
