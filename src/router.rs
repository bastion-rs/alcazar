use crate::{alcazar::AlcazarError, http_request::HttpError};
use std::str::FromStr;

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

    fn from_str(method: &str) -> Result<MethodType, AlcazarError> {
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

#[derive(Clone)]
pub struct Endpoint {
    method: MethodType,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            method: MethodType::GET,
        }
    }
}

impl Endpoint {
    pub fn new() -> Self {
        Endpoint::default()
    }

    pub fn add_method(mut self, method: MethodType) -> Self {
        self.method = method;
        self
    }
}

#[derive(Clone)]
pub struct Route {
    path: String,
    endpoint: Endpoint,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            path: "/".into(),
            endpoint: Endpoint::default(),
        }
    }
}

impl Route {
    pub fn new() -> Self {
        Route::default()
    }

    pub fn add_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn add_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    pub fn get_response(self) -> &'static str {
        "HTTP/1.1 200 OK\r\n\r\n"
    }
}

#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self { routes: Vec::new() }
    }
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub fn add_route(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn add_routes(mut self, routes: Vec<Route>) -> Self {
        for route in routes {
            self.routes.push(route);
        }
        self
    }

    pub fn get_handler(&self, method: MethodType, path: &str) -> Result<&Route, AlcazarError> {
        for route in &self.routes {
            if path == route.path && method == route.endpoint.method {
                return Ok(route);
            }
        }
        Err(AlcazarError::HttpError(HttpError::InternalServerError))
    }
}
