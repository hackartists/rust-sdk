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