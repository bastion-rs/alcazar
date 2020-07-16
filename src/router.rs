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

    pub fn add_method(&mut self, method: MethodType) -> &mut Self {
        self.method = method;
        self
    }

    pub fn build(&mut self) -> Self {
        Endpoint {
            method: self.method.clone(),
        }
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

    pub fn add_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }

    pub fn add_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;
        self
    }

    pub fn build(&mut self) -> Self {
        Route {
            path: self.path.clone(),
            endpoint: self.endpoint.clone(),
        }
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

    pub fn add_route(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn add_routes(&mut self, routes: Vec<Route>) -> &mut Self {
        for route in routes {
            self.routes.push(route);
        }
        self
    }

    pub fn build(&mut self) -> Self {
        Router {
            routes: self.routes.clone(),
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
