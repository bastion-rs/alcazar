use crate::{alcazar::AlcazarError, router::MethodType};
use httparse::{Error as HttparseError, Request, EMPTY_HEADER};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("partial content sended: status code 206")]
    PartialContent,
    #[error("internal server error: status code 500")]
    InternalServerError,
    #[error("method not implemented: status code 501")]
    MethodNotImplemented,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    HttparseError(#[from] HttparseError),
    #[error("method is missing in the request")]
    MethodMissing,
    #[error("path is missing in the request")]
    PathMissing,
}

pub struct HttpRequest {
    path: String,
    method: MethodType,
}

// See https://users.rust-lang.org/t/curl-post-tcpstream/38350/3 for understand how to handle a TcpStream as HttpRequest
impl HttpRequest {
    pub fn parse_stream(stream: &TcpStream) -> Result<HttpRequest, AlcazarError> {
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
            Err(_) => Err(AlcazarError::ParseError(ParseError::HttparseError(
                HttparseError::Status,
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

    fn parse_request(request: Request) -> Result<HttpRequest, AlcazarError> {
        let path = match request.path.map(String::from) {
            Some(path) => Ok(path),
            None => Err(AlcazarError::ParseError(ParseError::PathMissing)),
        }?;
        let method = match request.method {
            Some(method) => Ok(method),
            None => Err(AlcazarError::ParseError(ParseError::MethodMissing)),
        };
        let method = match method? {
            "POST" => Ok(MethodType::POST),
            "GET" => Ok(MethodType::GET),
            "PATCH" => Ok(MethodType::PATCH),
            "DELETE" => Ok(MethodType::DELETE),
            "CONNECT" => Ok(MethodType::CONNECT),
            "OPTIONS" => Ok(MethodType::OPTIONS),
            "TRACE" => Ok(MethodType::TRACE),
            "HEAD" => Ok(MethodType::HEAD),
            _ => Err(AlcazarError::HttpError(HttpError::MethodNotImplemented)),
        }?;
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
        let endpoint = Endpoint::default().add_method(MethodType::GET);
        let route = Route::new().add_path("/".into()).add_endpoint(endpoint);
        let router = Router::new().add_route(route);
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
