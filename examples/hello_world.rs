use alcazar::prelude::*;
use std::{io::Write, net::TcpStream, thread};

fn main() {
    let app = Alcazar::new().with_url("127.0.0.1:9000");
    thread::spawn(move || {
        app.start();
    });

    let stream = TcpStream::connect("127.0.0.1:9000");

    match stream {
        Ok(mut s) => {
            s.write("hello, world!".as_bytes()).unwrap();
            s.flush().unwrap();
        }
        Err(_) => {
            assert!(false);
        }
    }
}
