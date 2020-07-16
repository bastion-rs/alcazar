use crate::{http_request::HttpRequest, router::Router};
use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};
use tracing::info;

pub struct AppBuilder {
    url: SocketAddr,
    router: Router,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            url: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
            router: Router::default(),
        }
    }

    pub fn set_addr(&mut self, url: SocketAddr) -> &mut Self {
        self.url = url;
        self
    }

    pub fn set_router(&mut self, router: Router) -> &mut Self {
        self.router = router;
        self
    }

    pub fn start(&self) -> App {
        let listener = TcpListener::bind(self.url).unwrap();

        let local_addr = listener.local_addr().unwrap();
        let router = self.router.clone();

        info!("listening to {}", local_addr);
        std::thread::spawn(move || loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    // TODO: Stop to unwrap the world, set up a error handler
                    let http_request = HttpRequest::parse_stream(&stream).unwrap();
                    let handler = router.get_handler(http_request.method(), http_request.path());
                    // TODO: Router and middleware process, early return here for complete response
                    stream
                        .write_all(handler.unwrap().clone().get_response().as_bytes())
                        .unwrap();
                    stream.flush().unwrap();
                }
                Err(_) => info!("Client connexion failed."),
            }
        });

        App { local_addr }
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
        let alcazar = AppBuilder::new().set_addr(get_ipv4_socket_addr()).start();

        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            alcazar.local_addr().ip()
        );
    }

    #[test]
    fn add_url_ipv6() {
        let alcazar = AppBuilder::new().set_addr(get_ipv6_socket_addr()).start();

        assert_eq!("::1".parse::<IpAddr>().unwrap(), alcazar.local_addr().ip());
    }

    #[test]
    fn add_router() {
        let endpoint = Endpoint::new(MethodType::GET);
        let route = Route::new("/".into(), endpoint);
        let mut routes = Vec::new();
        routes.push(route);
        let router = Router::new(routes);
        let alcazar = AppBuilder::new().set_router(router).start();

        let mut stream = TcpStream::connect(alcazar.local_addr()).unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        let _ = reader.read_line(&mut buffer).unwrap();

        assert_eq!(buffer, "HTTP/1.1 200 OK\r\n");
    }

    // Test behind are both useless actually and need to be rework

    // #[test]
    // fn try_to_connect_ipv4() {
    //     let alcazar = AppBuilder::default()
    //         .set_addr(get_ipv4_socket_addr())
    //         .start();

    //     TcpStream::connect(alcazar.local_addr()).unwrap();
    // }

    // #[test]
    // fn try_to_connect_ipv6() {
    //     let alcazar = AppBuilder::default()
    //         .set_addr(get_ipv6_socket_addr())
    //         .start();

    //     TcpStream::connect(alcazar.local_addr()).unwrap();
    // }
}
