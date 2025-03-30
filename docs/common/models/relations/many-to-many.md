# Many to Many Relationship

This document describes how to define and manage **Many to Many** relationships in Biyard Rust projects using the `api_model` macro.

## Overview

A **Many to Many** relationship occurs when multiple entities can relate to multiple entities through a join (pivot) table.

For example, multiple `Post` entities can have multiple associated `Tag` entities, and vice versa.

## Basic Usage

Many to Many relationships are clearly defined using attributes provided by the `api_model` macro.

### Example

Here's a practical example where a `Post` can have multiple related `Tag` entities:

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/posts", table = posts)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,

    pub title: String,

    // Define Many to Many relationship: a post has multiple tags
    #[api_model(
        many_to_many = posts_tags,
        foreign_table_name = tags,
        foreign_primary_key = tag_id,
        foreign_reference_key = post_id
    )]
    pub tags: Vec<Tag>,
}

#[api_model(base = "/v1/tags", table = tags)]
pub struct Tag {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(unique)]
    pub tag: String,

    // Define Many to Many relationship: a tag has multiple posts
    #[api_model(
        many_to_many = posts_tags,
        foreign_table_name = posts,
        foreign_primary_key = post_id,
        foreign_reference_key = tag_id
    )]
    pub posts: Vec<Post>,
}
```

## Attribute Descriptions

| Attribute                | Description                                                    | Example                          |
|--------------------------|----------------------------------------------------------------|----------------------------------|
| `many_to_many`           | Name of the join (pivot) table managing the relationship       | `many_to_many = posts_tags`      |
| `foreign_table_name`     | Name of the related entity's table                             | `foreign_table_name = tags`      |
| `foreign_primary_key`    | Primary key of the related entity in the join table            | `foreign_primary_key = tag_id`   |
| `foreign_reference_key`  | Primary key of the current entity in the join table            | `foreign_reference_key = post_id`|
| `target_table`           | Determines source of relationship data (`join` or `foreign`)    | `target_table = join`            |

In the example above, the join table `posts_tags` manages the association between `Post` and `Tag`.

## Using `target_table`

You can specify the source table for retrieving relationship data using the `target_table` attribute. Setting `target_table` to `join` retrieves data from the join table, while `foreign` retrieves data directly from the related entity's table.

### Example

```rust
#[api_model(
    many_to_many = test_mtm_votes,
    type = JSONB,
    foreign_table_name = test_mtm_users,
    foreign_primary_key = user_id,
    foreign_reference_key = model_id,
    target_table = join
)]
pub user_votes: Vec<Vote>,

#[api_model(
    many_to_many = test_mtm_votes,
    type = JSONB,
    foreign_table_name = test_mtm_users,
    foreign_primary_key = user_id,
    foreign_reference_key = model_id,
    target_table = foreign
)]
pub users: Vec<User>,
```

In this example:
- `user_votes` retrieves data from the join table (`test_mtm_votes`).
- `users` retrieves data from the foreign table (`test_mtm_users`).

## Automatically Generated Features

- **Automatic JOINs**  
  Queries retrieving related data will automatically perform JOIN operations across the pivot table, simplifying complex relational data retrieval.

- **Intuitive API Client and Repository**  
  The `api_model` macro generates intuitive methods within API clients and repositories, making the handling of many-to-many relationships effortless.

## Recommendations and Best Practices

- Clearly define join tables (`many_to_many`) and related keys to avoid ambiguity.
- Be cautious about performance implications of many-to-many relations; optimize your queries to reduce database load.
- Consider explicitly defining and maintaining the join table structure to enhance clarity and control.

Following these guidelines will ensure clear, maintainable, and efficient management of Many to Many relationships in your Biyard Rust projects.
