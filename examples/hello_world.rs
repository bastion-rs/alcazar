use alcazar::prelude::*;
use std::{io::Write, net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr}, thread};
use std::{time::Duration, thread::park_timeout};

fn main() {
    let app = Alcazar::new().with_url(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    thread::spawn(move || {
        app.start();
    });

    let stream = TcpStream::connect("127.0.0.1:8080");

    match stream {
        Ok(mut s) => {
            s.write("hello, world!".as_bytes()).unwrap();
            s.flush().unwrap();
            park_timeout(Duration::from_secs(1));
            assert!(true);
        }
        Err(_) => {
            assert!(false);
        }
    }
}
