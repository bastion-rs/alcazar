pub mod alcazar;
pub mod error;
pub mod request;
pub mod router;
pub mod routing;
pub mod status_code;

pub mod prelude {
    pub use crate::alcazar::{App, AppBuilder};
    pub use crate::router::Router;
    // TODO: Remove endpoint later from public APIs
    pub use crate::routing::endpoint::Endpoint;
}
