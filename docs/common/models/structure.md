# Define a Structure

This document describes how to define structured data models within Biyard Rust projects using the custom `api_model` macro, streamlining serialization, validation, API interactions, and database integrations.

## 🛠️ Using the `api_model` Macro

The `api_model` macro supports two types of attributes:

- **Structure Attributes**: Applied to the entire struct, specifying API endpoints, supported actions, database tables, and response behaviors.
- **Field Attributes**: Applied individually to fields, specifying primary keys, relationships, and serialization rules.

This document specifically explains **structure attributes**.

---

## 📌 Structure Attributes

Structure attributes configure API endpoints, database associations, and model-specific actions:
- `api_model` automatically attaches derives of `Debug`, `Clone`, `serde::Deserialize`, `serde::Serialize`, `Default`, `PartialEq` by default.
- It also add `schemars::JsonSchema` and `aide::OperationIo` for `server` feature.

### Syntax Example:

```rust
#[api_model(base = "/v1/users", table = "users")]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    pub username: String,
}
```

---

### 📖 Attribute Definitions

| Attribute      | Description                                                             | Example                                         |
|----------------|-------------------------------------------------------------------------|-------------------------------------------------|
| `base`         | API path prefix for model-related endpoints.                            | `base = "/v1/users"`                            |
| `table`        | Specifies the database table associated with the model.                 | `table = "users"`                               |
| `queryable`    | Enables query functionality with optional sorting/filtering parameters. | `queryable = [(sort = Sorter)]`                 |
| `action`       | Defines actions (e.g., POST requests) with optional custom parameters.  | `action = [signup(code = String, param = u64)]` |
| `read_action`  | Defines custom read-only actions without parameters.                    | `read_action = read_data`                       |
| `action_by_id` | Defines actions targeted by entity ID (e.g., update, delete).           | `action_by_id = [get_data, update_data]`        |
| `response`     | Specifies custom response types for defined actions.                    | `response = [signup(UserResponse)]`             |

---

## 📖 Field Attributes

Field attributes specify behaviors at the individual field level:

| Attribute               | Description                                                              | Example                                                                  |
|-------------------------|--------------------------------------------------------------------------|--------------------------------------------------------------------------|
| `summary`               | Adds the field to `UserSummary`.                                         | `#[api_model(summary)]`                                                  |
| `queryable`             | Adds the field to `UserQuery`.                                           | `#[api_model(queryable)]`                                                |
| `action`                | Defines actions (e.g., POST requests) with optional custom parameters.   | `#[api_model(action = signup)]`                                          |
| `read_action`           | Adds the field to `UserQuery`.                                           | `#[api_model(read_action = read_data)]`                                  |
| `query_action`          | Adds the field to `UserQuery`.                                           | `#[api_model(query_action = list_data)]`                                 |
| `action_by_id`          | Defines actions targeted by entity ID (e.g., update, delete).            | `#[api_model(action_by_id = [get_data, update_data])]`                   |
| `primary_key`           | Marks the field as the primary key.                                      | `#[api_model(primary_key)]`                                              |
| `nullable`              | Allows the database field to accept null values.                         | `#[api_model(nullable)]`                                                 |
| `skip`                  | Excludes the field from serialization and database handling.             | `#[api_model(skip)]`                                                     |
| `type`                  | Describe type of SQL explicitly.                                         | `#[api_model(type = INTEGER)]`                                           |
| `nested`                | Defines nested data structures within the model.                         | `#[api_model(nested)]`                                                   |
| `one_to_many`           | Specifies a one-to-many relationship.                                    | `#[api_model(one_to_many = "posts", foreign_key = "author_id")]`         |
| `many_to_one`           | Specifies a many-to-one relationship.                                    | `#[api_model(many_to_one = "user")]`                                     |
| `many_to_many`          | Defines a many-to-many relationship via a `join table`.                  | `#[api_model(many_to_many = "posts_tags", foreign_table_name = "tags")]` |
| `aggregator`            | Embeds aggregation logic (sum, avg, etc.) into model fields.             | `#[api_model(aggregator = sum(price))]`                                  |
| `foreign_key`           | Indicates related key field name for `one_to_many` or `many_to_one`      | `#[api_model(one_to_many = "posts", foreign_key = "author_id")]`         |
| `foreign_table_name`    | Means foreign table name via joined table                                | `#[api_model(many_to_many = "posts_tags", foreign_table_name = "tags")]` |
| `foreign_primary_key`   | Indicates field name mapped to id field of foreign table in joined table | `#[api_model(foreign_primary_key = tag_id)]`                             |
| `foreign_reference_key` | Indicates field name mapped to id field of this table in joined table    | `#[api_model(foreign_reference_key = post_id)]`                          |

---

### ⚙️ Practical Struct Example:

Here's a complete, clearly structured Rust model:

```rust
#[api_model(
    base = "/v1/users",
    table = "users",
    queryable = [(sort = Sorter)],
    action = [signup(code = String, param = u64), login(test = Vec<String>)],
    read_action = read_data,
    action_by_id = [get_data, update_data],
    response = [signup(UserResponse)]
)]
#[derive(Debug, Clone, Default)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    pub username: String,

    #[api_model(nullable)]
    pub email: Option<String>,

    #[api_model(skip)]
    pub password_hash: String,
}
```

This setup ensures:

- Clear alignment of API paths and database table structures.
- Automatic generation of API clients and database CRUD operations.
- Custom request and response handling for specified actions.

---

### ✅ Recommended Best Practices

- Clearly define your `base` attribute to reflect your API structure.
- Always explicitly specify the associated `table` for clarity and database operations.
- Define custom parameters and response types for actions explicitly when required.

Following these best practices ensures your models are consistent, maintainable, and intuitive within the Biyard Rust ecosystem.

---

## 📌 Generated Summary Structure (`UserSummary`):

The macro identifies fields marked with `#[api_model(summary)]` and generates a simplified struct called `UserSummary`:

### 🚩 Automatically Generated Struct:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserSummary {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub nickname: Option<String>,
}
```

### 🎯 Purpose of the Summary Struct:

The `UserSummary` structure is designed to provide concise, efficient, and safe data exchange:

- **API List Responses**: Clearly optimized for list-based API responses (e.g., listing users without sensitive data).
- **Performance**: Reduces payload size and overhead by excluding non-summary fields.
- **Security**: Automatically excludes sensitive fields (e.g., `email`, `password`) not explicitly marked for summary.

---

## 📖 Rules for Generating Summary Structures:

- Fields **must** explicitly include the `summary` attribute to appear in the summary structure.
- The original model struct remains unaffected; the summary struct is an additional, simplified representation.
- Useful for API endpoints designed specifically for listing or simplified views.

---

## ✅ Recommended Best Practices:

- Clearly select summary fields carefully to exclude sensitive information.
- Leverage summary structures for efficient API responses.
- Regularly verify and adjust summary fields based on API usage patterns.

Following these best practices ensures safe, performant, and clear API data exchange within your Biyard Rust ecosystem.
