use crate::endpoint::{Endpoint, MethodType};
use crate::error::{AlcazarError, HttpError, Result};
use std::str::FromStr;

#[derive(Clone)]
pub struct Router {
    endpoints: Vec<Endpoint>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            endpoints: Vec::new(),
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub(crate) fn endpoints(&self) -> &Vec<Endpoint> {
        &self.endpoints
    }

    // TODO: Add handler parameter and set it up
    pub fn with_endpoint(mut self, path: &str, methods: &[&str]) -> Self {
        let endpoint = Endpoint::new()
            .with_methods(
                methods
                    .iter()
                    .map(|method| {
                        let fixed_method_name = method.trim().to_uppercase();
                        MethodType::from_str(&fixed_method_name)
                    })
                    .filter_map(|method| method.ok())
                    .collect(),
            )
            .with_path(path);
        self.endpoints.push(endpoint);
        self
    }

    pub fn include(mut self, router: &Router) -> Self {
        self.endpoints.extend(router.endpoints().iter().cloned());
        self
    }

    pub fn get_endpoint(&self, method: MethodType, path: &str) -> Result<&Endpoint> {
        for endpoint in &self.endpoints {
            if path == endpoint.path() && endpoint.methods().contains(&method) {
                return Ok(endpoint);
            }
        }
        Err(AlcazarError::HttpError(HttpError::InternalServerError))
    }
}
