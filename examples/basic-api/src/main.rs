use by_axum::axum::routing::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = by_axum::new().route("/test", get(test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    by_axum::serve(listener, app).await?;

    Ok(())
}

async fn test() -> String {
    "test".to_string()
}
