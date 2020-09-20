use crate::routing::pattern::PatternType;
use crate::{
    error::{AlcazarError, HttpError, Result},
    status_code::StatusCode,
};
use futures::future::{FutureExt, FutureObj, Shared};
use std::{future::Future, str::FromStr};

// TODO: Replace String in path for the 'a str type
// TODO: Mark the structure and methods as pub(crate) later
#[derive(Clone)]
pub struct Endpoint {
    pattern: PatternType,
    methods: Vec<MethodType>,
    handler: Shared<FutureObj<'static, StatusCode>>,
}

impl Endpoint {
    // Returns a default initialized endpoint instance.
    pub fn new<C, F>(path: &str, methods: Vec<MethodType>, handler: C) -> Result<Self>
    where
        C: Fn() -> F + Send + 'static,
        F: Future<Output = StatusCode> + Send + 'static,
    {
        let pattern = PatternType::from_str(path)?;
        let handler = FutureObj::new(Box::new(handler())).shared();
        Ok(Endpoint {
            pattern,
            methods,
            handler,
        })
    }

    // Returns a pattern against which can be checked match.
    pub fn pattern(&self) -> &PatternType {
        &self.pattern
    }

    // Returns a list of acceptable methods.
    pub fn methods(&self) -> &Vec<MethodType> {
        &self.methods
    }

    pub fn handler(&self) -> Shared<FutureObj<'static, StatusCode>> {
        self.handler.clone()
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
