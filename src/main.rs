use std::net::TcpListener;
use learning_rust::connection;

fn main() {
    let ip: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection::handle_connection(stream);
    }
}
