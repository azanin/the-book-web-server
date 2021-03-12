use std::net::TcpListener;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for incoming_stream in tcp_listener.incoming() {
        let incoming_stream = incoming_stream.unwrap();

        println!("Connection established!");
    }
}
