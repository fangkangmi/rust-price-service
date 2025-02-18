use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:8081").unwrap();
    stream.write("Hello, server!".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Server responded with: {:?}", 
        str::from_utf8(&buffer).unwrap()
    );
}
