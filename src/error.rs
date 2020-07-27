use httparse::Error as HttpParseError;
use std::io::Error as IOError;
use std::result;
use thiserror::Error;

// Alias for easier error handling and less boilerplate.
pub type Result<T> = result::Result<T, AlcazarError>;

#[derive(Error, Debug)]
pub enum AlcazarError {
    #[error(transparent)]
    IOError(#[from] IOError),
    #[error(transparent)]
    HttpError(#[from] HttpError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    RoutingError(#[from] RoutingError),
}

#[derive(Error, Debug, Clone)]
pub enum HttpError {
    #[error("partial content sended: status code 206")]
    PartialContent,
    #[error("internal server error: status code 500")]
    InternalServerError,
    #[error("method not implemented: status code 501")]
    MethodNotImplemented,
}

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error(transparent)]
    HttpParseError(#[from] HttpParseError),
    #[error("method is missing in the request")]
    MethodMissing,
    #[error("path is missing in the request")]
    PathMissing,
}

#[derive(Error, Debug, Clone)]
pub enum RoutingError {
    #[error("found an invalid {part:?} part of the {path:?} path.")]
    EndpointPathError { part: String, path: String },
}
