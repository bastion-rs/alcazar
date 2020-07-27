use crate::error::{AlcazarError, HttpError, Result};
use std::str::FromStr;

// TODO: Replace String in path for the 'a str type
// TODO: Mark the structure and methods as pub(crate) later
#[derive(Clone)]
pub struct Endpoint {
    path: String,
    methods: Vec<MethodType>,
}

impl<'a> Default for Endpoint {
    fn default() -> Self {
        Self {
            path: String::new(),
            methods: Vec::new(),
        }
    }
}

impl Endpoint {
    // Returns a default initialized endpoint instance.
    pub fn new() -> Self {
        Endpoint::default()
    }

    // Returns a path to the endpoint.
    pub fn path(&self) -> &String {
        &self.path
    }

    // Returns a list of acceptable methods.
    pub fn methods(&self) -> &Vec<MethodType> {
        &self.methods
    }

    // Overrides a list of acceptable methods by the endpoint.
    pub fn with_methods(mut self, methods: Vec<MethodType>) -> Self {
        self.methods = methods;
        self
    }

    // Overrides a path to the endpoint.
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
}

// TODO: Mark the enum as pub(crate) later
#[derive(PartialEq, Copy, Clone)]
pub enum MethodType {
    POST,
    GET,
    PATCH,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    HEAD,
}

impl FromStr for MethodType {
    type Err = AlcazarError;

    fn from_str(method: &str) -> Result<MethodType> {
        match method {
            "POST" => Ok(MethodType::POST),
            "GET" => Ok(MethodType::GET),
            "PATCH" => Ok(MethodType::PATCH),
            "DELETE" => Ok(MethodType::DELETE),
            "CONNECT" => Ok(MethodType::CONNECT),
            "OPTIONS" => Ok(MethodType::OPTIONS),
            "TRACE" => Ok(MethodType::TRACE),
            "HEAD" => Ok(MethodType::HEAD),
            _ => Err(AlcazarError::HttpError(HttpError::MethodNotImplemented)),
        }
    }
}
