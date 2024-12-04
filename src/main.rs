use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};


fn handle_request<'a>(request: String) -> (&'a str, String) {
    match &request[..] {
        "GET / HTTP/1.1" => {    
            (
                "HTTP/1.1 200 OK",
                fs::read_to_string("html/example.html").unwrap()
            )
        },
        "GET /Bana HTTP/1.1" => {
            (
                "HTTP/1.1 200 OK",
                fs::read_to_string("html/easter_egg.html").unwrap()
            )
        },
        _ => {
            (
                "HTTP/1.1 404 NOT FOUND",
                fs::read_to_string("html/404.html").unwrap()
            )
        }
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line= buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = handle_request(request_line);

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let ip: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
