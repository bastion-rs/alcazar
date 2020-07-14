use crate::http_request::HttpRequest;
use std::{
    io::Write,
    net::{SocketAddr, TcpListener},
};
use tracing::info;

#[derive(Default)]
pub struct AlcazarBuilder {
    url: Option<SocketAddr>,
}

impl AlcazarBuilder {
    pub fn set_addr(self, url: SocketAddr) -> Self {
        Self { url: url.into() }
    }

    pub fn start(self) -> Alcazar {
        let listener =
            TcpListener::bind(self.url.unwrap_or_else(|| "0.0.0.0:0".parse().unwrap())).unwrap();

        let local_addr = listener.local_addr().unwrap();

        info!("listening to {}", local_addr);
        std::thread::spawn(move || loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    // TODO: Stop to unwrap the world, set up a error handler
                    let http_request = HttpRequest::parse_stream(&stream).unwrap();
                    http_request.path();
                    http_request.method();
                    // TODO: Router and middleware process, early return here for complete response
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(_) => info!("Client connexion failed."),
            }
        });

        Alcazar { local_addr }
    }
}

pub struct Alcazar {
    local_addr: SocketAddr,
}

impl Alcazar {
    pub fn local_addr(&self) -> &SocketAddr {
        &self.local_addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
    }

    fn get_ipv6_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 0)
    }

    #[test]
    fn add_url_ipv4() {
        let alcazar = AlcazarBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .start();

        assert_eq!(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            alcazar.local_addr().ip()
        );
    }

    #[test]
    fn add_url_ipv6() {
        let alcazar = AlcazarBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .start();

        assert_eq!("::1".parse::<IpAddr>().unwrap(), alcazar.local_addr().ip());
    }

    #[test]
    fn try_to_connect_ipv4() {
        let alcazar = AlcazarBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .start();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }

    #[test]
    fn try_to_connect_ipv6() {
        let alcazar = AlcazarBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .start();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }
}
