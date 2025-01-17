# Biyard Macros

## API Model
### Usage
- `crate::Result` must be declared.
- For using `ApiError` as result error type, you should implement `From<String>` into inner error type
- If you want to use fully customized error type, you should implement `From<reqwest::Error>`.

``` rust
#[cfg(feature = "server")]
use by_axum::aide;

use by_macros::api_model;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[derive(Serialize, Deserialize)]
#[api_model(base = "/topics/v1", iter_type=Vec)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary, queryable)]
    pub description: String,
    #[api_model(summary, queryable)]
    pub status: i32,
    #[api_model(summary)]
    pub created_at: i64,

    pub updated_at: i64,
    pub tags: Vec<String>,
}

```

### Expanded code

``` rust
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary, queryable)]
    pub description: String,
    #[api_model(summary, queryable)]
    pub status: i32,
    #[api_model(summary)]
    pub created_at: i64,

    pub updated_at: i64,
    pub tags: Vec<String>,
}

impl Topic {
    pub fn get_client(endpoint: &str) -> TopicClient {
        TopicClient { endpoint : endpoint.to_string() }
    }
}

pub struct TopicSummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i64,
}

pub struct TopicQuery {
    pub size: usize,
    pub bookmark: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
}

pub struct TopicClient {
    pub endpoint: String,
}

impl TopicClient {
    pub async fn query(&self, params:TopicQuery) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }

    pub async fn get(&self, id: &str) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::get(& endpoint).await
    }
}
```
