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

#[derive(Clone, Serialize, Deserialize)]
#[api_model(base = "/topics/v1", iter_type=Vec)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, queryable, read_action = search_by, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update)]
    pub status: i32,
    #[api_model(summary, read_action = [search_by, date_from])]
    pub created_at: i64,
    pub is_liked: bool,

    pub updated_at: i64,
    #[api_model(action_by_id = like, related = CommentRequest)]
    #[api_model(action = comment, related = Comment)]
    pub comments: Vec<Comment>,

    #[api_model(action_by_id = update)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommentRequest {
    pub comment_id: String,
    pub is_liked: bool,
}
```

### Expanded code

``` rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicAction {
    Create(TopicCreateRequest),
    Comment(Comment),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicCreateRequest {
    pub title: String,
    pub description: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicByIdAction {
    Like(CommentRequest),
    Update(TopicUpdateRequest),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicUpdateRequest {
    pub description: String,
    pub status: i32,
    pub tags: Vec<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicSummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicQuery {
    pub size: usize,
    pub bookmark: Option<String>,
    pub action: Option<TopicReadActionType>,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub created_at: Option<i64>,
}
impl TopicQuery {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            ..Self::default()
        }
    }
    pub fn with_bookmark(mut self, bookmark: String) -> Self {
        self.bookmark = Some(bookmark);
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }
    pub fn search_by(mut self, description: String, created_at: i64) -> Self {
        self.description = Some(description);
        self.created_at = Some(created_at);
        self.action = Some(TopicReadActionType::SearchBy);
        self
    }
    pub fn date_from(mut self, created_at: i64) -> Self {
        self.created_at = Some(created_at);
        self.action = Some(TopicReadActionType::DateFrom);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicReadActionType {
    SearchBy,
    DateFrom,
}
impl Topic {
    pub fn get_client(endpoint: &str) -> TopicClient {
        TopicClient {
            endpoint: endpoint.to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct TopicClient {
    pub endpoint: String,
}
impl TopicClient {
    pub async fn query(&self, params: TopicQuery) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn get(&self, id: &str) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::get(&endpoint).await
    }
    pub async fn act(&self, params: TopicAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/action", self.endpoint, "/topics/v1");
        rest_api::post(&endpoint, params).await
    }
    pub async fn act_by_id(&self, id: &str, params: TopicByIdAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}/action", self.endpoint, "/topics/v1", id);
        rest_api::post(&endpoint, params).await
    }
}
```
