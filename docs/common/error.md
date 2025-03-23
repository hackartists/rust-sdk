# Error Handling

This document describes the standardized approach to defining and managing errors within Biyard Rust projects using the custom `Error` enum. It provides structured error definitions, translation support for internationalization (i18n), and integration with common external error types.

## Error Enum Definition

Errors are centralized in the `common/error.rs` file, defined as an enum with descriptive variants, each supporting multilingual translation using the `Translate` macro.

### Example Definition

```rust
// packages/common/error.rs
use bdk::prelude::*;

#[derive(Debug, serde::Serialize, PartialEq, Eq, serde::Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Error {
    #[translate(
        ko = "회원가입에 실패했습니다. 다시 시도해주세요.",
        en = "Sign-up failed. Please try again."
    )]
    SignupFailed(String),

    #[translate(
        ko = "API 호출에 실패하였습니다. 네트워크 연결상태를 확인해주세요.",
        en = "Failed to call API. Please check network status."
    )]
    ApiCallError(String),

    #[translate(ko = "입력값이 잘못되었습니다.", en = "Invalid input value.")]
    ValidationError(String),

    #[translate(
        ko = "데이터베이스 쿼리에 실패하였습니다. 입력값을 확인해주세요.",
        en = "Failed to execute database query. Please check your inputs."
    )]
    DatabaseError(String),
}
```

## Explanation of Variants

Each variant clearly represents a specific error scenario with a descriptive message:

- **`SignupFailed`**: Indicates failure during the user sign-up process.
- **`ApiCallError`**: Represents errors arising from failed external API calls.
- **`ValidationError`**: Captures errors resulting from invalid user input or validation failures.
- **`DatabaseError`**: Indicates failure during database operations.

All variants carry a descriptive error message (`String`) for detailed context.

## Translation Support

Each error variant uses the custom `#[translate]` macro to provide clear, user-friendly multilingual error messages. This simplifies internationalization (i18n) and ensures users receive clear, language-specific feedback.

**Example**:

```rust
#[translate(
    ko = "입력값이 잘못되었습니다.",
    en = "Invalid input value."
)]
ValidationError(String)
```

## Conversions from External Errors

The custom error enum provides automatic conversions from common external crate errors (`reqwest`, `validator`, `sqlx`), simplifying error handling throughout the application:

```rust
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ApiCallError(e.to_string())
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}
```

These conversions streamline error propagation and allow consistent handling of external library errors.

## Server Integration (Axum Response)

The error enum integrates seamlessly with the Axum web framework (when the `server` feature is enabled) to provide standardized HTTP responses:

```rust
#[cfg(feature = "server")]
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            _ => StatusCode::BAD_REQUEST,
        };

        let body = Json(self);

        (status_code, body).into_response()
    }
}
```

This ensures consistent HTTP response formatting for errors, simplifying frontend error handling.

## Recommended Usage Pattern

When defining API endpoints or business logic functions, leverage the custom `Result` alias to clearly communicate potential errors:

```rust
use common::prelude::*;

pub async fn create_user(user: User) -> Result<User> {
    if user.username.is_empty() {
        return Err(Error::ValidationError("Username cannot be empty.".into()));
    }

    // Database operation that could fail, automatically converting errors
    let saved_user = save_to_db(user).await?;

    Ok(saved_user)
}
```
