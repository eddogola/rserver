use std::io::prelude::*; // bring in used io traits(e.g Reader and Writer) to namespace
use std::net::TcpListener;
use std::net::TcpStream; // for type declaration
use std::time::Duration;
use std::thread;
use std::fs;

use mic_check::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // unwrap here for demo purposes - just to get it to work
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let expected_get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    let header_200 = "200 OK";
    let html_file = fs::read_to_string("hello.html").unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer));
    if buffer.starts_with(expected_get) {
        write_response(&stream, header_200, html_file);
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        write_response(&stream, header_200, html_file);
    } else {
        let header_404 = "404 NOT FOUND";
        write_response(&stream, header_404, String::from("Page not found"));
    }

    stream.flush().unwrap();
}

fn write_response(mut stream: &TcpStream, header: &str, contents: String) {
    let response = format!(
                            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                            header,
                            contents.len(),
                            contents);
    stream.write(response.as_bytes()).unwrap();
}