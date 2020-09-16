use httparse::Error as HttpParseError;
use std::io::Error as IOError;
use std::result;
use thiserror::Error;

// Alias for easier error handling and less boilerplate.
pub type Result<T> = result::Result<T, AlcazarError>;

struct WrapIOError(Box<IOError>);

#[derive(Error, Debug, Clone)]
pub enum AlcazarError {
    #[error("partial content sended: status code 206")]
    WrapIOError,
    #[error(transparent)]
    HttpError(#[from] HttpError),
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error(transparent)]
    RoutingError(#[from] RoutingError),
}

// impl Display for WrapIOError {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match *self {
//             DoubleError::EmptyVec =>
//                 write!(f, "please use a vector with at least one element"),
//             // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
//             DoubleError::Parse(ref e) => e.fmt(f),
//         }
//     }
// }

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
    InvalidPathError { part: String, path: String },
    #[error("can't compile {0} regex for the given path.")]
    RegexCompileError(String),
}
