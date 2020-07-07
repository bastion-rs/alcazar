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
}

#[cfg(test)]
mod tests {
    use super::Alcazar;

    #[test]
    fn add_url() {
        let app = Alcazar::new().with_url("127.0.0.1");

        assert!(app.url == String::from("127.0.0.1"));
    }
}
