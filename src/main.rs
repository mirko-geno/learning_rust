use std::net::TcpListener;
use learning_rust::{connection, thread_pool::ThreadPool};

fn main() {
    let ip: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();

    let pool = ThreadPool::new(1);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| connection::handle_connection(stream));
    }
}
