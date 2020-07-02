// rust

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn ls() {
    let ls = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in ls.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn main() {
    println!("Hello, world!");
    let lest = longest("abc", "xyz");
    println!("longest is {}", lest);
    ls();
}
