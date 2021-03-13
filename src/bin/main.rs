use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;
use the_book_web_server::ThreadPool;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for incoming_stream in tcp_listener.incoming() {
        let incoming_stream = incoming_stream.unwrap();

        pool.execute(|| {
            handle_connection(incoming_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, file_name) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

    let response = format!(
        "{}{}",
        status,
        fs::read_to_string(file_name).unwrap()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
