# Relations

In Biyard Rust projects, data models frequently represent complex relationships between database entities. The `api_model` macro simplifies managing these relations, automatically generating the necessary database queries and handling serialization seamlessly.

This section describes how to clearly define and manage relational structures within your data models, supporting various relational scenarios such as one-to-many, many-to-one, and many-to-many.

## Supported Relation Types

Biyard Rust projects explicitly support the following relation types:

- **[One to Many](./one-to-many.md)**
  Represents a relationship where one entity relates to multiple entities. For example, one user can have many posts.

- **[Many to Many](./many-to-many.md)**
  Represents a relationship where multiple entities can associate with multiple other entities through a join table. For example, posts associated with multiple tags.

## Defining Relations with `api_model`

Relations are defined directly within your model structs using attributes provided by the `api_model` macro:

### Example

```rust
use bdk::prelude::*;

#[api_model(base = "/v1/users", table = users)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    pub username: String,

    // One-to-many relation: a user has multiple posts
    #[api_model(one_to_many = posts, foreign_key = author_id)]
    pub posts: Vec<Post>,
}

#[api_model(base = "/v1/posts", table = posts)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,

    pub title: String,

    // Many-to-one relation: a post belongs to one user
    #[api_model(many_to_one = users)]
    pub author_id: i64,

    // Many-to-many relation: a post has multiple tags
    #[api_model(many_to_many = posts_tags, foreign_table_name = tags, foreign_primary_key = tag_id, foreign_reference_key = post_id)]
    pub tags: Vec<Tag>,
}

#[api_model(base = "/v1/tags", table = tags)]
pub struct Tag {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(unique)]
    pub tag: String,

    // Many-to-many relation: a tag has multiple posts
    #[api_model(many_to_many = posts_tags, foreign_table_name = posts, foreign_primary_key = post_id, foreign_reference_key = tag_id)]
    pub posts: Vec<Post>,
}
```


