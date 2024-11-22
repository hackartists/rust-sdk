use std::error::Error;

use ::axum::{Extension, Json};
use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};

pub use aide::*;

#[cfg(feature = "lambda")]
pub mod lambda_adapter;

pub mod logger;

pub fn new() -> ApiRouter {
    let app = ApiRouter::new().api_route("/version", get(version));

    app
}

pub async fn serve(
    _tcp_listener: tokio::net::TcpListener,
    app: ApiRouter,
) -> Result<(), Box<dyn Error>> {
    let mut api = OpenApi {
        info: Info {
            description: Some("Open API Specification".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let app = app
        .route("/spec", get(serve_api))
        .finish_api(&mut api)
        .layer(Extension(api));

    #[cfg(not(feature = "lambda"))]
    let _ = axum::serve(_tcp_listener, app.into_make_service()).await?;

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

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
