pub mod alcazar;
pub mod endpoint;
pub mod error;
pub mod request;
pub mod router;

pub mod prelude {
    pub use crate::alcazar::{App, AppBuilder};
    // TODO: Remove endpoint later from public APIs
    pub use crate::endpoint::Endpoint;
    pub use crate::router::Router;
}
