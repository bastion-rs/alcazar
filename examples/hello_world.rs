use alcazar::prelude::*;
use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};
use std::{thread::park_timeout, time::Duration};

fn main() {
    let alcazar = AppBuilder::new()
        .set_addr(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
        .start();

    let mut stream = TcpStream::connect(alcazar.local_addr()).unwrap();

    stream.write_all(b"hello, world!").unwrap();
    stream.flush().unwrap();
    park_timeout(Duration::from_secs(1));
}
