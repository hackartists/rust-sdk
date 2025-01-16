pub use crate::router::BiyardRouter as Router;
pub use aide::axum::routing;
pub use axum::*;

pub mod response {
    pub use aide::axum::IntoApiResponse as IntoResponse;
    pub use axum::response::*;
}
