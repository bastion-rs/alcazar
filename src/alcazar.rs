use std::{io::{BufRead, BufReader}, net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr}};
use tracing::info;

pub struct Alcazar {
    url: SocketAddr,
}

impl Alcazar {
    pub fn new() -> Self {
        Alcazar { url: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080) }
    }

    pub fn with_url(mut self, url: impl Into<SocketAddr>) -> Self {
        self.url = url.into();
        self
    }

    pub fn start(self) {
        let listener = TcpListener::bind(&self.url).unwrap();

        info!("Alcazar: Start listening on: {}", &self.url);
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let buffer = BufReader::new(stream);

                    for line in buffer.lines() {
                        println!("{}", line.unwrap());
                    }
                    info!("Client connected from: {}", addr);
                }
                Err(_) => info!("Client connexion failed."),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Alcazar;
    use std::{net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr, Ipv6Addr}, thread};

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn get_ipv6_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)), 8080)
    }

    fn create_app(url: SocketAddr) {
        let app = Alcazar::new().with_url(url);
        thread::spawn(move || {
            app.start();
        });
    }

    #[test]
    fn add_url_ipv4() {
        let app = Alcazar::new().with_url(get_ipv4_socket_addr());
        assert!(app.url == get_ipv4_socket_addr());
    }

    #[test]
    fn add_url_ipv6() {
        let app = Alcazar::new().with_url(get_ipv6_socket_addr());
        assert!(app.url == get_ipv6_socket_addr());
    }

    #[test]
    fn try_to_connect_ipv4() {
        create_app(get_ipv4_socket_addr());

        match TcpStream::connect("127.0.0.1:8080") {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn try_to_connect_ipv6() {
        create_app(get_ipv6_socket_addr());

        match TcpStream::connect("[::1]:8080") {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
