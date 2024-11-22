use axum::{routing::get, Router};

pub use axum;

#[cfg(feature = "lambda")]
pub mod lambda_adapter;

pub mod logger;

pub fn new() -> Router {
    let app = Router::new().route("/version", get(version));

    app
}

pub async fn serve(
    _tcp_listener: tokio::net::TcpListener,
    app: Router,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "lambda"))]
    let _ = axum::serve(_tcp_listener, app).await?;

    #[cfg(feature = "lambda")]
    {
        lambda_runtime::run(lambda_adapter::LambdaAdapter::from(app.into_service()))
            .await
            .unwrap();
    }

    Ok(())
}

async fn version() -> String {
    match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => format!("{}", version),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string()
}
