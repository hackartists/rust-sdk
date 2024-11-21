use std::net::TcpListener;

use axum::{routing::get, serve::Serve, Router};

pub fn new<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let app = Router::new().route("/version", get(version));

    #[cfg(feature = "local")]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    app
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
