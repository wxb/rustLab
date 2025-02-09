#[allow(dead_code)]

enum Language {
    Rust,
    TypeScript,
    Java,
    Golang,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Java => "java",
            Language::Golang => "go",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref())
}

fn main() {
    print_ref("Hello world");

    print_ref(String::from("Hello world"));

    let lang = Language::Rust;
    print_ref(lang);
}
