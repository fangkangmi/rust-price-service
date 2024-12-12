use std::net::TcpListener;
use std::io::{Read, Write};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    println!("Running on port 8081");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        // A mutable buffer of 1024 bytes, initialized with zeros.
        // This buffer can be used to store data temporarily during I/O operations.
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();

    }
}
