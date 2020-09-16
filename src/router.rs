use crate::{
    error::{AlcazarError, HttpError, Result},
};
use crate::{
    routing::endpoint::{Endpoint, MethodType},
    status_code::StatusCode,
};
use std::{future::Future, str::FromStr};

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

    // Returns a list of declared endpoints.
    pub(crate) fn endpoints(&self) -> &Vec<Endpoint> {
        &self.endpoints
    }

    // TODO: Add handler parameter and set it up
    pub fn with_endpoint<C, F>(mut self, path: &str, methods: &[&str], exec: C) -> Self
    where
        C: Fn() -> F + Send + Sync + 'static,
        F: Future<Output = Result<StatusCode>> + Send + Sync + 'static,
    {
        let acceptable_methods = methods
            .iter()
            .map(|method| {
                let fixed_method_name = method.trim().to_uppercase();
                MethodType::from_str(&fixed_method_name)
            })
            .filter_map(|method| method.ok())
            .collect();
        match Endpoint::new(path, acceptable_methods, exec) {
            Ok(endpoint) => {
                self.endpoints.push(endpoint);
            }
            Err(err) => println!("{:?}, The endpoint has been skipped.", err),
        };

        self
    }

    // Merges two routers together.
    pub fn include(mut self, router: &Router) -> Self {
        self.endpoints.extend(router.endpoints().iter().cloned());
        self
    }

    // Returns an endpoint by the given path and the method.
    pub fn get_endpoint(&self, method: MethodType, path: &str) -> Result<&Endpoint> {
        for endpoint in &self.endpoints {
            let pattern = endpoint.pattern();
            if pattern.is_match(path) && endpoint.methods().contains(&method) {
                return Ok(endpoint);
            }
        }
        Err(AlcazarError::HttpError(HttpError::InternalServerError))
    }
}
