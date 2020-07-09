use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub struct Url {
    _path: String,
    _endpoint: String,
    _parameters: Option<HashMap<String, String>>,
}

pub enum HttpMethod {
    _POST,
    _GET,
    _PATCH,
    _DELETE,
    _CONNECT,
    _OPTIONS,
    _TRACE,
    _HEAD,
}

pub struct HttpVersion {
    _major: u8,
    _minor: u8,
}

pub struct HttpParser {
    _method: HttpMethod,
    _url: Url,
    _http_version: HttpVersion,
}

impl HttpParser {
    pub fn parse(stream: TcpStream) {
        let buffer = BufReader::new(stream);

        for line in buffer.lines() {
            println!("{}", line.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alcazar::Alcazar;
    use std::{io::Write, net::TcpStream, thread};

    #[test]
    fn send_hello_world() {
        let app = Alcazar::new().with_url("127.0.0.1:9000");
        thread::spawn(move || {
            app.start();
        });

        let stream = TcpStream::connect("127.0.0.1:9000");

        match stream {
            Ok(mut s) => {
                s.write("hello, world!".as_bytes()).unwrap();
                s.flush().unwrap();
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
