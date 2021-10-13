fn main() {}

#[cfg(test)]
mod tests_threads {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn multi_thread() {
        thread::spawn(move || {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn multi_thread_join() {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束
        handle.join().unwrap();
    }

    #[test]
    fn multi_thread_join_variants() {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束
        // main 现场阻塞直到handle代表的子线程执行完毕退出后才继续往下执行main线程
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
}
