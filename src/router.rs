#[derive(PartialEq, Clone)]
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

#[derive(Default, Clone)]
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
    pub fn new() -> Self {
        Router::default()
    }

    pub fn add_route(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn get_handler(&self, method: MethodType, path: String) -> Option<&Route> {
        for route in &self.routes {
            if path == route.path && method == route.endpoint.method {
                return Some(route);
            }
        }
        None
    }
}

impl Route {
    pub fn new() -> Self {
        Route::default()
    }

    pub fn set_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn set_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    pub fn get_response(self) -> &'static str {
        "HTTP/1.1 200 OK\r\n\r\n"
    }
}

impl Endpoint {
    pub fn new() -> Self {
        Endpoint::default()
    }

    pub fn set_method(mut self, method: MethodType) -> Self {
        self.method = method;
        self
    }
}
