mod sound;

fn main() {
    // 绝对路径
    crate::sound::instrument::clarinet();

    // 相对路径
    sound::instrument::clarinet();
}
