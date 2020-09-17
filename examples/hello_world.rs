use alcazar::prelude::*;
use alcazar::{error::Result, status_code::StatusCode};
use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};
use std::{thread::park_timeout, time::Duration};

async fn handler() -> Result<StatusCode> {
    Ok(StatusCode::Ok)
}

fn main() {
    let router = Router::new().with_endpoint("/", &["get"], handler);
    let alcazar = AppBuilder::default()
        .set_addr(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
        .set_router(router)
        .start()
        .unwrap();

    let mut stream = TcpStream::connect(alcazar.local_addr()).unwrap();

    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    match reader.read_line(&mut buffer) {
        Ok(_n) => {
            if buffer.starts_with("HTTP/1.1 200 OK\r\n") {
                println!("Hello, world!");
            }
        }
        Err(_) => println!("Goodbye, world!"),
    }

    park_timeout(Duration::from_secs(1));
}
