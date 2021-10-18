#![allow(unused)]

use hello::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::{fs, io::prelude::*, thread, time::Duration};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(b"GET /hello HTTP/1.1") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(15));
        ("HTTP/1.1 200 OK", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
