mod alcazar;
mod http_request;
mod router;

pub mod prelude {
    pub use crate::alcazar::{App, AppBuilder};
    pub use crate::router::{Endpoint, MethodType, Route, Router};
}
