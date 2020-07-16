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

#[derive(Clone)]
pub struct Endpoint {
    method: MethodType,
}

#[derive(Default, Clone)]
pub struct Route {
    path: String,
    endpoint: Endpoint,
}

#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            method: MethodType::GET,
        }
    }
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Router {
            routes
        }
    }

    pub fn get_handler(&self, method: MethodType, path: &str) -> Option<&Route> {
        for route in &self.routes {
            if path == route.path && method == route.endpoint.method {
                return Some(route);
            }
        }
        None
    }
}

impl Route {
    pub fn new(path: String, endpoint: Endpoint) -> Self {
        Route {
            path,
            endpoint
        }
    }

    pub fn get_response(self) -> &'static str {
        "HTTP/1.1 200 OK\r\n\r\n"
    }
}

impl Endpoint {
    pub fn new(method: MethodType) -> Self {
        Endpoint {
            method
        }
    }
}
