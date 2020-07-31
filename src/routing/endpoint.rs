use crate::error::{AlcazarError, HttpError, Result};
use crate::request::HttpRequest;
use crate::routing::pattern::PatternType;
use std::str::FromStr;

// TODO: Replace String in path for the 'a str type
// TODO: Mark the structure and methods as pub(crate) later
#[derive(Clone)]
pub struct Endpoint {
    pattern: PatternType,
    methods: Vec<MethodType>,
}

impl Endpoint {
    // Returns a default initialized endpoint instance.
    pub fn new(path: &str, methods: Vec<MethodType>) -> Result<Self> {
        let pattern = PatternType::from_str(path)?;
        Ok(Endpoint { pattern, methods })
    }

    // Returns a pattern against which can be checked match.
    pub fn pattern(&self) -> &PatternType {
        &self.pattern
    }

    // Returns a list of acceptable methods.
    pub fn methods(&self) -> &Vec<MethodType> {
        &self.methods
    }

    // TODO: Remove this method and call handler instead
    pub fn get_response(&self, _request: &HttpRequest) -> &'static str {
        "HTTP/1.1 200 OK\r\n\r\n"
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
