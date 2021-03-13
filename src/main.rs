use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for incoming_stream in tcp_listener.incoming() {
        let incoming_stream = incoming_stream.unwrap();

        handle_connection(incoming_stream);

        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer);
    let str = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", str)
}
