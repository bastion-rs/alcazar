use httparse::{Request, EMPTY_HEADER};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct HttpRequest {}

// https://users.rust-lang.org/t/curl-post-tcpstream/38350/3
impl HttpRequest {
    pub fn parse_stream(mut stream: TcpStream) {
        // Creating the reader and the buffer for read the stream
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();
        // Read the stream line by line and add it to the buffer
        loop {
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    println!("TcpStream -> EOF means the connection was terminated.");
                    break;
                }
                Ok(_n) => {
                    if buffer.ends_with("\r\n\r\n") {
                        println!("HttpRequest -> \\r\\n alone means end of request.");
                        break;
                    }
                }
                _ => (),
            }
        }
        // Create headers for parse the request with the crate Httparse
        let mut headers = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut headers[..]);
        // Getting the status of the request
        let request_status = request.parse(buffer.as_ref()).unwrap();
        // If the request is complete we are returning the response in the stream
        if request_status.is_complete() {
            match request.path {
                Some(ref _path) => {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                None => {
                    println!("You get bonkerized.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alcazar::Alcazar;
    use std::{
        io::{BufRead, BufReader, Write},
        net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
        thread,
    };

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn create_app(url: SocketAddr) {
        let app = Alcazar::new().with_url(url);
        thread::spawn(move || {
            app.start();
        });
    }

    #[test]
    fn parse_stream() {
        create_app(get_ipv4_socket_addr());
        let mut stream = TcpStream::connect(get_ipv4_socket_addr()).unwrap();
        stream.write(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        match reader.read_line(&mut buffer) {
            Ok(_n) => {
                assert_eq!(buffer, "HTTP/1.1 200 OK\r\n");
            }
            _ => (),
        }
    }
}
