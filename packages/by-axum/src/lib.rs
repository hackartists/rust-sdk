use axum::routing::get;

pub use axum;
pub use logger as log;
use router::BiyardRouter;

#[cfg(feature = "lambda")]
pub mod lambda_adapter;
pub mod logger;
pub mod router;

pub fn new() -> BiyardRouter {
    BiyardRouter::new().route("/version", get(version))
}

pub async fn serve(
    _tcp_listener: tokio::net::TcpListener,
    app: BiyardRouter,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = app.layer(tower_http::cors::CorsLayer::permissive());

    #[cfg(not(feature = "lambda"))]
    axum::serve(_tcp_listener, app.inner).await?;

    #[cfg(feature = "lambda")]
    {
        lambda_runtime::run(lambda_adapter::LambdaAdapter::from(
            app.inner.into_service(),
        ))
        .await
        .unwrap();
    }

    Ok(())
}

async fn version() -> String {
    match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => version.to_string(),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string()
}
