pub mod auth;
pub mod axum;
mod docs;
use std::sync::Arc;

use ::axum::{Extension, Json, Router};
pub use logger as log;
use router::BiyardRouter;

#[cfg(feature = "lambda")]
pub mod lambda_adapter;
pub mod logger;
pub mod router;
pub use aide;
pub mod rest_api_adapter;

pub use by_types::ApiError;
pub type Result<T, E> = std::result::Result<Json<T>, ApiError<E>>;
pub use schemars;
use tracing_subscriber::EnvFilter;

pub fn new() -> BiyardRouter {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(option_env!("RUST_LOG").unwrap_or("info")))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    BiyardRouter::new()
}

pub fn finishing(app: BiyardRouter) -> Router {
    let mut api = app.open_api;
    app.inner
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
}

pub async fn serve(
    _tcp_listener: tokio::net::TcpListener,
    app: BiyardRouter,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app = app.layer(tower_http::cors::CorsLayer::permissive());
    let mut api = app.open_api;
    let app = app
        .inner
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)));

    #[cfg(not(feature = "lambda"))]
    axum::serve(_tcp_listener, app).await?;

    #[cfg(feature = "lambda")]
    {
        lambda_runtime::run(lambda_adapter::LambdaAdapter::from(app.into_service()))
            .await
            .unwrap();
    }

    Ok(())
}

pub fn into_api_adapter(app: BiyardRouter) -> rest_api_adapter::RestApiAdapter {
    let app = finishing(app);
    rest_api_adapter::RestApiAdapter::new(app)
}
