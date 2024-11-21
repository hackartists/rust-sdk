use std::error::Error;

use axum::{routing::get, Router};

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
) -> Result<(), Box<dyn Error>> {
    #[cfg(not(feature = "lambda"))]
    let _ = axum::serve(_tcp_listener, app);

    #[cfg(feature = "lambda")]
    {
        lambda_runtime::run(lambda_adapter::LambdaAdapter::from(app))
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
