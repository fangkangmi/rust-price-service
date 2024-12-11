use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str::{self, FromStr};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 2083];
            stream.read(&mut read_buffer).unwrap();

            let request_str = match str::from_utf8(&read_buffer) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Failed to convert buffer to string: {}", e);
                    continue;
                }
            };

            let request = match HttpRequest::from_str(request_str) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Failed to parse HTTP request: {}", e);
                    continue;
                }
            };
            
            Router::route(request, &mut stream)
        }
    }
}