use crate::error::{AlcazarError, HttpError, Result};
use crate::routing::pattern::PatternType;
use std::{future::Future, pin::Pin, str::FromStr, sync::Arc};

// TODO: Replace String in path for the 'a str type
// TODO: Mark the structure and methods as pub(crate) later
pub struct Endpoint {
    pattern: PatternType,
    methods: Vec<MethodType>,
    handler: Arc<Exec>,
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Self {
            pattern: self.pattern.clone(),
            methods: self.methods.clone(),
            handler: Arc::clone(&self.handler),
        }
    }
}

pub struct Init(pub Box<dyn Fn() -> Exec + Send + Sync>);
pub struct Exec(pub Pin<Box<dyn Future<Output = Result<()>> + Send + Sync>>);

impl Init {
    pub(crate) fn new<C, F>(init: C) -> Self
    where
        C: Fn() -> F + Send + Sync + 'static,
        F: Future<Output = Result<()>> + Send + Sync + 'static,
    {
        let init = Box::new(move || {
            let fut = init();
            let exec = Box::pin(fut);

            Exec(exec)
        });

        Init(init)
    }
}

impl Endpoint {
    // Returns a default initialized endpoint instance.
    pub fn new(path: &str, methods: Vec<MethodType>, handler: Init) -> Result<Self> {
        let pattern = PatternType::from_str(path)?;
        Ok(Endpoint {
            pattern,
            methods,
            handler: Arc::new((handler.0)()),
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

    // TODO: Remove this method and call handler instead
    pub fn handler(&self) -> &'static str {
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
