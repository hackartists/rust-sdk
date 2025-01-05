use app::App;

pub use axum;
pub use logger as log;

pub mod app;
#[cfg(feature = "lambda")]
pub mod lambda_adapter;
pub mod logger;

pub fn new() -> App {
    App::new()
}

pub async fn serve(
    tcp_listener: tokio::net::TcpListener,
    app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    app.serve(tcp_listener).await
}
