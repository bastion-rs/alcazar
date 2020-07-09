use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub enum MethodType {
    POST,
    GET,
    PATCH,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    HEAD,
}

pub struct HttpRequest {}

impl HttpRequest {
    pub fn parse_stream(stream: TcpStream) {
        let buffer = BufReader::new(stream);

        for line in buffer.lines() {
            let line = line.unwrap();
            let sublines: Vec<&str> = line.split(' ').collect();

            let method = match sublines[0] {
                "POST" => MethodType::POST,
                "GET" => MethodType::GET,
                "PATCH" => MethodType::PATCH,
                "DELETE" => MethodType::DELETE,
                "CONNECT" => MethodType::CONNECT,
                "OPTIONS" => MethodType::OPTIONS,
                "TRACE" => MethodType::TRACE,
                "HEAD" => MethodType::HEAD,
                _ => panic!("TODO: handle error"),
            };
        }
    }
}
