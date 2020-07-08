use std::net::TcpListener;

use tracing::debug;

pub struct Alcazar {
    url: String,
}

impl Alcazar {
    pub fn new() -> Self {
        let url = String::default();

        Alcazar { url }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn start(self) {
        let listener = TcpListener::bind(&self.url).unwrap();

        debug!("Alcazar: Start listening on: {}", &self.url);
        loop {
            match listener.accept() {
                Ok((client, addr)) => {
                    debug!("Client connected from: {}", addr);
                }
                Err(_) => debug!("Client connexion failed."),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Alcazar;
    use std::{net::TcpStream, thread};

    #[test]
    fn add_url() {
        let app = Alcazar::new().with_url("127.0.0.1:9000");

        assert!(app.url == String::from("127.0.0.1:9000"));
    }

    #[test]
    fn try_to_connect() {
        let app = Alcazar::new().with_url("127.0.0.1:9000");
        thread::spawn(move || {
            app.start();
        });

        match TcpStream::connect("127.0.0.1:9000") {
            Ok(_) => {
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
