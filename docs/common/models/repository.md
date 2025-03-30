# Repository

A **Repository** encapsulates the logic required to access, modify, and manage data stored in a database. It abstracts database operations into clearly defined methods, enhancing readability, maintainability, and efficiency.

## Defining API Model

```rust
#[api_model(base = "/v1/users", table = "users")]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(summary, read_action = get_by_name, act_by_id = update_name)]
    pub username: String,
    pub password: String,
}
```

### Generated key structures

It generates structures necessary for database operations and API interactions:

| Structure                     | Description                                   |
|-------------------------------|-----------------------------------------------|
| `User`                        | Core database model                           |
| `UserRepository`              | Handles CRUD operations for `User`            |
| `UserRepositoryUpdateRequest` | Struct for defining update requests           |

#### User Methods

| Method            | Description                                  |
|-------------------|----------------------------------------------|
| `get_repository`  | Creates a repository instance for the model. |

#### UserRepository Methods

| Method              | Description                                                   |
|---------------------|---------------------------------------------------------------|
| `create_this_table` | Creates the model's database table only.                      |
| `create_table`      | Creates the model's database table and related tables.        |
| `insert`            | Inserts a new record into the database.                       |
| `update`            | Updates an existing record based on provided parameters.      |
| `delete`            | Deletes a record by its ID.                                   |
| `insert_with_tx`    | Inserts records within a transaction context.                 |
| `update_with_tx`    | Updates records within a transaction context.                 |
| `delete_with_tx`    | Deletes records within a transaction context.                 |
| `find`              | Retrieves multiple records based on query parameters.         |
| `find_one`          | Retrieves a single record based on specific query parameters. |

## Using Repository

### Getting Started

Initialize the PostgreSQL pool before using the repository:

```rust,no_run
let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect(
        option_env!("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
    )
    .await
    .unwrap();
```

### Creating Tables

Create tables required by your model:

```rust
let repo = User::get_repository(pool);
repo.create_table().await?;
```

### Inserting Data

Basic insert operation:

```rust
let username = "name".to_string();
let repo = User::get_repository(pool);

let user = repo.insert(username).await?;
```

Insert with transaction:

```rust
let mut tx = repo.pool.begin().await?;
for i in 0..3 {
    repo.insert_with_tx(&mut *tx, format!("user {i}")).await?;
}
tx.commit().await?;
```

### Updating Data

#### Basic Updates

Update using predefined action:

```rust
let id = 1;
let request = UserByIdAction::UpdateName(UserUpdateNameRequest {
    name: "new_name".to_string(),
});

let repo = User::get_repository(pool);
let param = request.into();
let user = repo.update(id, param).await?;
```

#### Customizing Update Requests

Customizing updates with builder methods:

```rust
let id = 1;
let request = UserByIdAction::UpdateName(UserUpdateNameRequest {
    name: "new_name".to_string(),
});

let repo = User::get_repository(pool);
let param: UserRepositoryUpdateRequest = request.into();
let param = param.with_password("new_password".to_string());

let user = repo.update(id, param).await?;
```

### Deleting Data

Basic delete operation:

```rust
let repo = User::get_repository(pool);
let user = repo.delete(1).await?;
```

Delete using transaction:

```rust
let mut tx = repo.pool.begin().await?;
let user = repo.delete_with_tx(&mut *tx, 1).await?.ok_or(Error::NoUser)?;
tx.commit().await?;
```

### Retrieving Data

Repository provides two straightforward retrieval methods: `find_one` and `find`.

- Recommended usage for simple `AND`-`Equals` queries based on `ReadAction` or `Query`.
- For complex queries, use [QueryBuilder](./query-builder.md).

#### Retrieve Single Record

To fetch a single record matching a specific query:

```rust
let username = "name".to_string();
let request = UserReadAction::new().get_by_name(username);

let repo = User::get_repository(pool);
let user = repo.find_one(&request).await?;
```

This retrieves a single `User` instance that matches the provided query (`username`).

#### Retrieve Multiple Records

To fetch multiple records with pagination:

```rust
let request = UserQuery::new(5).with_page(1);

let repo = User::get_repository(pool);
let users = repo.find(&request).await?;
```

This retrieves multiple `UserSummary` records based on the specified query (`size` and `page`). Useful for paginated responses and listings.
