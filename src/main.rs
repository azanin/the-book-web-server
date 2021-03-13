use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for incoming_stream in tcp_listener.incoming() {
        let incoming_stream = incoming_stream.unwrap();
        handle_connection(incoming_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status, file_name) = if buffer.starts_with(get) {
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
