use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let expected_get = b"GET / HTTP/1.1";

    println!("Request: {}", String::from_utf8_lossy(&buffer));
    if buffer.starts_with(expected_get) {
        let html_file = fs::read_to_string("hello.html").unwrap();
        write_response(&stream, html_file);
    } else {
        write_response(&stream, String::from("Page not found"));
    }

    stream.flush().unwrap();
}

fn write_response(mut stream: &TcpStream, contents: String) {
    let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            contents.len(),
                            contents);
    stream.write(response.as_bytes()).unwrap();
}