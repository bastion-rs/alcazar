use crate::{
    http_request::{HttpError, HttpRequest, ParseError},
    router::Router,
};
use std::{
    io::{Error as IOError, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum AlcazarError {
    #[error(transparent)]
    IOError(#[from] IOError),
    #[error(transparent)]
    HttpError(#[from] HttpError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
}

pub struct AppBuilder {
    addr: SocketAddr,
    router: Router,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
            router: Router::default(),
        }
    }
}

impl AppBuilder {
    pub fn set_addr(&mut self, addr: SocketAddr) -> &mut Self {
        self.addr = addr;
        self
    }

    pub fn set_router(&mut self, router: Router) -> &mut Self {
        self.router = router;
        self
    }

    pub fn start(&self) -> Result<App, AlcazarError> {
        let listener = TcpListener::bind(self.addr)?;
        let local_addr = listener.local_addr()?;
        let router = self.router.clone();

        info!("listening to {}", local_addr);
        std::thread::spawn(move || -> Result<(), AlcazarError> {
            loop {
                match listener.accept() {
                    Ok((mut stream, _addr)) => {
                        let http_request = HttpRequest::parse_stream(&stream)?;
                        let handler =
                            router.get_handler(http_request.method(), http_request.path())?;
                        stream.write_all(handler.clone().get_response().as_bytes())?;
                        stream.flush()?;
                    }
                    Err(_) => info!("Client connection failed."),
                }
            }
        });

        Ok(App { local_addr })
    }
}

pub struct App {
    local_addr: SocketAddr,
}

impl App {
    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::{Endpoint, MethodType, Route};
    use std::{
        io::{BufRead, BufReader},
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream},
    };

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
    }

    fn get_ipv6_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 0)
    }

    #[test]
    fn add_url_ipv4() {
        let endpoint = Endpoint::default().add_method(MethodType::GET);
        let route = Route::new().add_path("/".into()).add_endpoint(endpoint);
        let router = Router::new().add_route(route);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            alcazar.local_addr().ip()
        );
    }

    #[test]
    fn add_url_ipv6() {
        let endpoint = Endpoint::default().add_method(MethodType::GET);
        let route = Route::new().add_path("/".into()).add_endpoint(endpoint);
        let router = Router::new().add_route(route);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        assert_eq!("::1".parse::<IpAddr>().unwrap(), alcazar.local_addr().ip());
    }

    #[test]
    fn add_router() {
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

    #[test]
    fn try_to_connect_ipv4() {
        let endpoint = Endpoint::default().add_method(MethodType::GET);
        let route = Route::new().add_path("/".into()).add_endpoint(endpoint);
        let router = Router::new().add_route(route);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }

    #[test]
    fn try_to_connect_ipv6() {
        let endpoint = Endpoint::default().add_method(MethodType::GET);
        let route = Route::new().add_path("/".into()).add_endpoint(endpoint);
        let router = Router::new().add_route(route);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }
}
