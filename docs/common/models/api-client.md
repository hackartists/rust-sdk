# Define API Client

The `api_model` macro provides a convenient way to generate HTTP API clients based on the specified `base` attribute, facilitating seamless frontend-backend interactions.

## Definition of API Client

### Pre-requisites

#### Define a Result type in crate root

In the root of the `common` crate, implement a generic `Result` type:

```rust
pub type Result<T> = std::result::Result<T, crate::error::Error>;
```

#### Define an Error type

The custom `Error` type must implement the following traits. For more details, refer to [Error Handling](../error.md).

| Trait                             | Description                                    |
|-----------------------------------|------------------------------------------------|
| `From<sqlx::Error>`               | Database error handling                        |
| `From<reqwest::Error>`            | HTTP request error handling                    |
| `IntoResponse`                    | Converts errors into HTTP responses            |
| `serde::Serialize`                | Serialization support                          |
| `serde::Deserialize`              | Deserialization support                        |
| `schemars::JsonSchema` for server | JSON schema generation for documentation       |
| `aide::OperationIo` for server    | API documentation generation                   |

## Using `api_model`

Below is a basic definition for generating an API client:

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users")]
pub struct User {
    pub id: i64,
    pub created_at: i64,
    pub username: String,
    pub password: String,
}
```

- Generates a static method `get_client(endpoint: &str)` returning `UserClient`.

### Basic Usage

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let cli = User::get_client("https://api.example.com");
    let user_id = 6;

    // GET /v1/users/6
    let user = cli.get(user_id).await?;

    // GET /v1/users?size=10
    let users = cli.query(UserQuery::new(10)).await?;

    Ok(())
}
```

## Using `summary`

Fields annotated with `summary` generate a simplified summary type (`UserSummary`) to exclude sensitive information.

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users")]
pub struct User {
    #[api_model(summary)]
    pub id: i64,
    #[api_model(summary)]
    pub created_at: i64,
    #[api_model(summary)]
    pub username: String,
    pub password: String,
}
```

### Usage

Query responses use `UserSummary`:

```rust
#[derive(Debug, serde::Serialize, PartialEq, Eq, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserSummary {
    pub id: i64,
    pub created_at: i64,
    pub username: String,
}
```

## API actions

`api_model` supports conventional API actions.

### Queryable

Adding `queryable` enables fields as query parameters.

```rust
#[api_model(base = "/v1/users")]
pub struct User {
    #[api_model(summary, queryable)]
    pub username: String,
}
```

### Usage

```rust
cli.query(UserQuery::new(10).with_username("name".to_string())).await?;
```

Generated Query Struct:

```rust
pub struct UserQuery {
    pub size: usize,
    pub bookmark: Option<String>,
    pub username: Option<String>,
}
```

### Query Actions

Custom actions for queries:

```rust
#[api_model(summary, query_action = search_by)]
pub email: String,
```

### Usage

```rust
cli.query(UserQuery::new(10).with_page(1).search_by("a@example.com".to_string())).await?;
```

Generated Struct:

```rust
pub struct UserQuery {
    pub size: usize,
    pub action: Option<UserQueryActionType>,
    pub bookmark: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
```

### Read Actions

Custom read actions that return a single model:

```rust
#[api_model(summary, queryable, read_action = find_by)]
pub username: String,
```

### Usage

```rust
cli.find_by("name".to_string(), "a@example.com".to_string()).await?;
```

Generated Structs:

```rust
pub struct UserReadAction {
    pub action: UserReadActionType,
    pub username: Option<String>,
    pub email: Option<String>,
}

pub enum UserParam {
    Query(UserQuery),
    Read(UserReadAction),
}

pub enum UserGetResponse {
    Query(QueryResponse<UserSummary>),
    Read(User),
}
```

### Actions

Actions (`signup`, `login`) perform operations via `POST` requests:

```rust
#[api_model(action = [signup, login])]
pub password: String,
```

### Usage

```rust
cli.signup("name".to_string(), "hash".to_string(), "a@example.com".to_string()).await?;
cli.login("name".to_string(), "hash".to_string()).await?;
```

Generated Structs:

```rust
pub enum UserAction {
    Signup(UserSignupRequest),
    Login(UserLoginRequest),
}

pub struct UserSignupRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}
```

### Action by id

Similar to `action`, but uses entity IDs in requests (`PUT`, `DELETE` methods):

```rust
#[api_model(action_by_id = [get_data, update_data])]
pub email: String,
```

### Usage

```rust
cli.update_data(user_id, "a@example.com".to_string()).await?;
```

Generated Structs:

```rust
pub enum UserActionById {
    GetData,
    UpdateData(UserUpdateRequest),
}

pub struct UserUpdateRequest {
    pub email: String,
}
```

## Generated structures

| Structure         | Description                             |
|-------------------|-----------------------------------------|
| `User`            | Core data model                         |
| `UserClient`      | API client for making HTTP calls        |
| `UserQuery`       | Struct for building query parameters    |
| `UserParam`       | Enum for query or read actions          |
| `UserGetResponse` | Enum representing query/read responses  |
| `UserSummary`     | Simplified struct for listing responses |
| `UserReadAction`  | Struct for single entity read actions   |
| `UserAction`      | Enum defining general POST actions      |
| `UserActionById`  | Enum defining ID-based actions          |

