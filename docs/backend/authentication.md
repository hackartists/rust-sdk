# Authentication

## `with_auth_cookie` function

The `with_auth_cookie` function is used to facilitate authentication for subsequent client requests by attaching an authentication cookie to the HTTP response headers.

This function sets the `SET-COOKIE` header in the HTTP response, allowing the client to store the authentication cookie for subsequent requests.

**Supporting Scheme**:
```rust
pub enum TokenScheme {
    Bearer,
    Usersig,
    XServerKey,
    Secret,
}
```

**Example**:
```rust
// api/main.rs
#[tokio::main]
async fn main() -> Result<()> {
    let app = make_app().await?;
    let port = config::get().port;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("{YOUR_CLIENT_DOMAIN}"))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION, COOKIE]);
    let app = app.layer(cors_layer);
    by_axum::serve_wo_cors_layer(listener, app).await.unwrap();

    Ok(())
}

// Store the generated JWT (which serves as a Bearer token) in an authentication cookie.
// The TokenScheme::Bearer argument indicates the type of the token being stored,
async fn login(
      &self,
      ...
   ) -> Result<JsonWithHeaders<User>> {
      // Retrieve or create a user object
      let user: User = /* Some logic to fetch or create the user */;

      // Generate a JWT token for the user
      let jwt = AppClaims::generate_token(&user)?;

      // Attach the token as a Bearer token in the authentication cookie
      Ok(
         JsonWithHeaders::new(user)
         .with_auth_cookie(TokenScheme::Bearer, &jwt)
      )
   }
```