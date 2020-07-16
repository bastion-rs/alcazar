use crate::router::MethodType;
use httparse::{Request, EMPTY_HEADER};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};
use tracing::info;

pub struct HttpRequest {
    path: String,
    method: MethodType,
}

// See https://users.rust-lang.org/t/curl-post-tcpstream/38350/3 for understand how to handle a TcpStream as HttpRequest
impl HttpRequest {
    pub fn parse_stream(stream: &TcpStream) -> Option<HttpRequest> {
        // Creating the reader and the buffer for read the stream
        let mut reader = BufReader::new(stream);
        let mut buffer = String::new();
        // Read the stream line by line and add it to the buffer
        loop {
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    info!("TcpStream -> EOF means the connection was terminated.");
                    break;
                }
                Ok(_n) => {
                    if buffer.ends_with("\r\n\r\n") {
                        info!("HttpRequest -> \\r\\n alone means end of request.");
                        break;
                    }
                }
                Err(error) => info!("An error occured during request parsing: {}", error),
            }
        }
        // Create headers for parse the request with the crate Httparse
        let mut headers = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut headers[..]);
        // Getting the status of the request
        let request_status = request.parse(buffer.as_ref()).unwrap();
        // If the request is complete we are returning the response in the stream
        if request_status.is_complete() {
            info!("Request is complete.");
            let path = request.path.map(String::from).unwrap();
            let method = match request.method.unwrap() {
                "POST" => MethodType::POST,
                "GET" => MethodType::GET,
                "PATCH" => MethodType::PATCH,
                "DELETE" => MethodType::DELETE,
                "CONNECT" => MethodType::CONNECT,
                "OPTIONS" => MethodType::OPTIONS,
                "TRACE" => MethodType::TRACE,
                "HEAD" => MethodType::HEAD,
                _ => MethodType::GET,
            };
            Some(HttpRequest { path, method })
        } else {
            // TODO: Determine what to do if request is not complete
            info!("Request is partial.");
            None
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    pub fn method(&self) -> MethodType {
        self.method
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alcazar::AppBuilder,
        router::{Endpoint, MethodType, Route, Router},
    };
    use std::{
        io::{BufRead, BufReader, Write},
        net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    };

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
    }

    #[test]
    fn parse_stream() {
        let endpoint = Endpoint::new().add_method(MethodType::GET).build();
        let route = Route::new()
            .add_path("/".into())
            .add_endpoint(endpoint)
            .build();
        let router = Router::new().add_route(route).build();
        let alcazar = AppBuilder::new()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router.clone())
            .start();

        let mut stream = TcpStream::connect(alcazar.local_addr()).unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        let _ = reader.read_line(&mut buffer).unwrap();

        assert_eq!(buffer, "HTTP/1.1 200 OK\r\n");
    }
}
