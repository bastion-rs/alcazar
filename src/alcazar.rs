use crate::error::Result;
use crate::request::HttpRequest;
use crate::router::Router;

use std::io::Write;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    time::Duration,
};

// use futures::future::BoxFuture;

use tracing::info;

use bastion_executor::prelude::*;
use lightproc::prelude::ProcStack;

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

    pub fn start(&self) -> Result<App> {
        let listener = TcpListener::bind(self.addr).expect("unwrap listener");
        let local_addr = listener.local_addr().expect("unwrap local_addr");
        let router = self.router.clone();

        info!("listening to {}", local_addr);
        std::thread::spawn(move || -> Result<()> {
            std::thread::sleep(Duration::new(1, 0));
            loop {
                match listener.accept() {
                    Ok((mut stream, _addr)) => {
                        let request = HttpRequest::parse_stream(&stream)?;
                        let endpoint = router.get_endpoint(request.method(), request.path())?;
                        // TODO: Call the endpoint's handler and write the response back
                        let handler = run(async { endpoint.handler().await }, ProcStack::default());

                        stream
                            .write_all(
                                handler
                                    .expect("unwrap handler")
                                    .into_string_response()
                                    .as_slice(),
                            )
                            .expect("unwrap write_all");
                        stream.flush().expect("unwrap flush");
                    }
                    Err(_) => println!("Client connection failed."),
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

    use crate::status_code::StatusCode;

    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};

    fn get_ipv4_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0)
    }

    fn get_ipv6_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 0)
    }

    async fn handler() -> Result<StatusCode> {
        Ok(StatusCode::Ok)
    }

    #[test]
    fn add_url_ipv4() {
        let router = Router::new().with_endpoint("/", &["get"], handler);
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
        let router = Router::new().with_endpoint("/", &["get"], handler);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        assert_eq!("::1".parse::<IpAddr>().unwrap(), alcazar.local_addr().ip());
    }

    #[test]
    fn add_router() {
        let router = Router::new().with_endpoint("/", &["get"], handler);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router)
            .start()
            .expect("unwrap appbuilder");

        let mut stream = TcpStream::connect(alcazar.local_addr()).expect("unwrap connect");
        stream
            .write_all(b"GET / HTTP/1.1\r\n\r\n")
            .expect("unwrap write_all test");
        stream.flush().expect("unwrap flush test");

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        let _ = reader
            .read_line(&mut buffer)
            .expect("unwrap read_line test");

        assert_eq!(buffer, "HTTP/1.1 200 OK\r\n");
    }

    #[test]
    fn try_to_connect_ipv4() {
        let router = Router::new().with_endpoint("/", &["get"], handler);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv4_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }

    #[test]
    fn try_to_connect_ipv6() {
        let router = Router::new().with_endpoint("/", &["get"], handler);
        let alcazar = AppBuilder::default()
            .set_addr(get_ipv6_socket_addr())
            .set_router(router)
            .start()
            .unwrap();

        TcpStream::connect(alcazar.local_addr()).unwrap();
    }
}
