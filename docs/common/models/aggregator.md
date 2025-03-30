# Aggregator

This document explains how to define and utilize aggregator fields (`sum`, `max`, `min`, `avg`, `exist`, `count`) in Biyard Rust projects using the `api_model` macro.

## Overview

Aggregator fields allow models to include aggregated values from related entities, such as counting children or calculating sums, averages, maximums, and minimums.

## Defining Aggregator Fields

You define aggregator fields directly in your models with the `aggregator` attribute in the `api_model` macro:

### Example

Here's a practical example using a `User` model and a related `Post` model:

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users", table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    pub username: String,

    // Count of posts associated with the user
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = count)]
    pub num_of_posts: i64,

    // Sum of views from posts
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = sum(views))]
    pub total_post_views: i64,

    // Maximum views on a single post
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = max(views))]
    pub max_post_views: i64,

    // Minimum views on a single post
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = min(views))]
    pub min_post_views: i64,

    // Average views across posts
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = avg(views))]
    pub avg_post_views: f64,

    // Boolean indicating if the user has any posts
    #[api_model(one_to_many = posts, foreign_key = author_id, aggregator = exist)]
    pub has_posts: bool,
}

#[api_model(base = "/v1/posts", table = posts)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,

    pub title: String,

    pub views: i64,

    #[api_model(many_to_one = users)]
    pub author_id: i64,
}
```

## Supported Aggregators

| Aggregator | Description                               | Example               | Returned Type |
|------------|-------------------------------------------|-----------------------|---------------|
| `sum`      | Sum of a numeric field                    | `aggregator=sum(views)`      | numeric       |
| `max`      | Maximum value of a numeric field          | `aggregator=max(views)`      | numeric       |
| `min`      | Minimum value of a numeric field          | `aggregator=min(views)`      | numeric       |
| `avg`      | Average of a numeric field                | `aggregator=avg(views)`      | floating-point|
| `count`    | Count of related records                  | `aggregator=count`            | integer       |
| `exist`    | Checks existence of any related record    | `aggregator=exist`            | boolean       |

## Usage Example

Here's how you might query aggregated fields:

```rust
let user = User::query_builder()
    .id_equals(user_id)
    .query()
    .map(User::from)
    .fetch_one(&pool)
    .await?;

println!("Total posts: {}", user.num_of_posts);
println!("Total views: {}", user.total_post_views);
println!("Max post views: {}", user.max_post_views);
println!("Min post views: {}", user.min_post_views);
println!("Average views per post: {:.2}", user.avg_post_views);
println!("Has posts: {}", user.has_posts);
```

## Recommendations and Best Practices

- Clearly define the aggregator fields you need to avoid unnecessary performance overhead.
- Ensure that fields targeted by aggregators (`sum`, `max`, `min`, `avg`) are numeric types.
- Optimize your queries to avoid aggregating large datasets unnecessarily.

By following these guidelines, you can efficiently leverage aggregators in your Biyard Rust projects.
