use crate::error::{AlcazarError, HttpError, ParseError, Result};
use crate::routing::endpoint::MethodType;
use httparse::{Error as HttpParseError, Request, EMPTY_HEADER};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;
use tracing::info;

pub struct HttpRequest {
    path: String,
    method: MethodType,
}

// See https://users.rust-lang.org/t/curl-post-tcpstream/38350/3 for understand how to handle a TcpStream as HttpRequest
impl HttpRequest {
    pub(crate) fn parse_stream(stream: &TcpStream) -> Result<HttpRequest> {
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
        let request_status = match request.parse(buffer.as_ref()) {
            Ok(request_status) => Ok(request_status),
            Err(_) => Err(AlcazarError::ParseError(ParseError::HttpParseError(
                HttpParseError::Status,
            ))),
        }?;
        // If the request is complete we are returning the response in the stream
        if request_status.is_complete() {
            info!("Request is complete.");
            HttpRequest::parse_request(request)
        } else {
            Err(AlcazarError::HttpError(HttpError::PartialContent))
        }
    }

    fn parse_request(request: Request) -> Result<HttpRequest> {
        let path = match request.path.map(String::from) {
            Some(path) => Ok(path),
            None => Err(AlcazarError::ParseError(ParseError::PathMissing)),
        }?;
        let method = match request.method {
            Some(method) => Ok(method),
            None => Err(AlcazarError::ParseError(ParseError::MethodMissing)),
        }?;
        let method = MethodType::from_str(method)?;

        Ok(HttpRequest { path, method })
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
    use crate::alcazar::AppBuilder;
    use crate::router::Router;
    use std::io::{BufRead, BufReader, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
    }

    #[test]
    fn parse_stream() {
        let router = Router::new().with_endpoint("/", &["get"]);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        let mut stream = TcpStream::connect(alcazar.local_addr()).unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        let _ = reader.read_line(&mut buffer).unwrap();

        assert_eq!(buffer, "HTTP/1.1 200 OK\r\n");
    }
}
