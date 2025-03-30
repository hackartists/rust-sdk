# One to Many Relationship

This document describes how to define and manage **One to Many** relationships in Biyard Rust projects using the `api_model` macro.

## Overview

A **One to Many** relationship occurs when a single entity can relate to multiple entities.

For example, a single `User` can have multiple `Post` entities associated with it.

## Basic Usage

One to Many relationships are defined clearly using attributes provided by the `api_model` macro.

### Example

Here's a practical example where a `User` has multiple related `Post` entities:

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users", table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    pub username: String,

    // Define a One to Many relationship: a user has multiple posts
    #[api_model(one_to_many = posts, foreign_key = author_id)]
    pub posts: Vec<Post>,
}

#[api_model(base = "/v1/posts", table = posts)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,

    pub title: String,

    // Define Many to One relationship: each post belongs to a single user
    #[api_model(many_to_one = users)]
    pub author_id: i64,
}
```

> **Important**: The `one_to_many` attribute should always be paired with a corresponding `many_to_one` attribute to establish a clear and consistent relationship.

## Attribute Descriptions

### One to Many

| Attribute      | Description                                             | Example                        |
|----------------|---------------------------------------------------------|--------------------------------|
| `one_to_many`  | Name of the related table to establish the relationship | `one_to_many = posts`          |
| `foreign_key`  | Foreign key field referencing the parent entity         | `foreign_key = author_id`      |

In the example above, the `posts` field in the `User` model references the `author_id` field in the `Post` table.

### Many to One

| Attribute     | Description                                            | Example               |
|---------------|--------------------------------------------------------|-----------------------|
| `many_to_one` | Name of the parent table to establish the relationship | `many_to_one = users` |

In the example above, the `author_id` field in the `Post` model references the parent `User` table.

## Automatically Generated Features

- **Automatic JOINs**  
  Queries retrieving related data will automatically perform JOIN operations, simplifying data retrieval across related tables.

- **Intuitive API Client and Repository**  
  The `api_model` macro generates appropriate methods within API clients and repositories, making related data management effortless.

## Data Retrieval Examples

Here's how to retrieve related data using `UserRepository`:

```rust
let repo = User::get_repository(pool);

// Retrieve a specific user and their posts
let user = repo.find_one(&UserReadAction::new().with_id(1)).await?;
let posts: Vec<Post> = user.posts;
```

Or using the generated API client:

```rust
let cli = User::get_client("https://api.example.com");

// Retrieve a user and their associated posts by user ID
let user = cli.get(1).await?;
let posts = user.posts;
```

## Recommendations and Best Practices

- Always explicitly specify the `foreign_key` when defining a One to Many relationship.
- Be mindful of performance implications when retrieving large amounts of data; consider selecting specific fields or limiting query results.
- Utilize a dedicated Query Builder for optimized queries, particularly with large datasets.
- Always pair a `one_to_many` attribute with a corresponding `many_to_one` attribute to ensure relationship integrity.

Following these guidelines will ensure clear, maintainable, and efficient management of One to Many relationships in your Biyard Rust projects.
