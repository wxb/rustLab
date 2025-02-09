use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    arc_mutext_is_send_sync();
}

// Arc<MutexM<T>>是可以多线程共享且修改数据
fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();

    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a={:?}", a);
}
