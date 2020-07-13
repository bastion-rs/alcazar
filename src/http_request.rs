use httparse::{Request, EMPTY_HEADER};
use std::{io::Read, net::TcpStream};

pub struct HttpRequest {}

impl HttpRequest {
    pub fn parse_stream(mut stream: TcpStream) {
        let mut headers = [EMPTY_HEADER; 0];
        let mut req = Request::new(&mut headers[..]);

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        req.parse(buffer.as_ref());
        println!("{:?}", req);
    }
}
