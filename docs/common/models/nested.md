# Nesting Relation Fields

This document explains how to use the `nested` attribute to perform nested joins for retrieving related entities in Biyard Rust projects using the `api_model` macro.

## Overview

By default, related entities retrieved through a model are empty unless explicitly specified. To include data from nested relationships, use the `nested` attribute with the generated `query_builder`, enabling powerful multi-level queries.

## Defining Nested Relationships

Use the `nested` attribute explicitly to indicate fields that should be loaded with nested related entities.

### Example

Here's an example demonstrating nested relationship definitions using `User`, `Post`, and `Tag`:

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users", table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    pub username: String,

    // One-to-many nested relation: User to Posts
    #[api_model(one_to_many = posts, foreign_key = author_id, nested)]
    pub posts: Vec<Post>,
}

#[api_model(base = "/v1/posts", table = posts)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,

    pub title: String,

    #[api_model(many_to_one = users)]
    pub author_id: i64,

    // Many-to-many nested relation: Posts to Tags
    #[api_model(
        many_to_many = posts_tags,
        foreign_table_name = tags,
        foreign_primary_key = tag_id,
        foreign_reference_key = post_id,
    )]
    pub tags: Vec<Tag>,
}

#[api_model(base = "/v1/tags", table = tags)]
pub struct Tag {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(unique)]
    pub tag: String,
}
```

## Nested Queries Using Query Builder

Utilize methods on your model's query builder, such as `posts_builder` or `tags_builder`, to perform nested queries clearly and efficiently.

### Nested Query Example

```rust
let result = User::query_builder()
    .id_equals(user_id)
    .posts_builder(Post::query_builder())
    .query()
    .map(User::from)
    .fetch_one(&pool)
    .await?;
```

In this query:

- The `posts_builder` method allows nested queries on the `posts` field.
  - Conditions such as `.title_contains` optimize data retrieval.

## Automatically Generated Features

- **Automatic Nested JOINs**  
  The query builder constructs necessary JOIN statements automatically, simplifying complex data retrieval.

- **Intuitive Query Builder Interface**  
  Specify complex nested queries easily with builder methods provided by the `api_model` macro.

## Recommendations and Best Practices

- Clearly specify the `nested` attribute for fields requiring nested data retrieval.
- Limit deeply nested queries to essential data only to maintain performance.
- Apply precise conditions within nested builders for optimized query execution.

Following these guidelines will help ensure efficient and maintainable nested queries in your Biyard Rust projects.

