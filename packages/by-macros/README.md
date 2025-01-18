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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/topics/v1", iter_type=Vec, read_action = no_param_action)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(read_action = user_info)]
    pub wallet_address: String,
    #[api_model(read_action = [check_email,user_info])]
    pub email: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, queryable, query_action = search_by, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update, read_action = user_info)]
    pub status: i32,
    #[api_model(summary, query_action = [search_by, date_from])]
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

#### Expanded code

``` rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Topic {
    pub id: String,
    pub wallet_address: String,
    pub email: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i64,
    pub is_liked: bool,
    pub updated_at: i64,
    pub comments: Vec<Comment>,
    pub tags: Vec<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicAction {
    Comment(Comment),
    Create(TopicCreateRequest),
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
    Update(TopicUpdateRequest),
    Like(CommentRequest),
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
    pub action: Option<TopicQueryActionType>,
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
        self.action = Some(TopicQueryActionType::SearchBy);
        self
    }
    pub fn date_from(mut self, created_at: i64) -> Self {
        self.created_at = Some(created_at);
        self.action = Some(TopicQueryActionType::DateFrom);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicQueryActionType {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicReadAction {
    pub action: Option<TopicReadActionType>,
    pub wallet_address: Option<String>,
    pub email: Option<String>,
    pub status: Option<i32>,
}
impl TopicReadAction {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn user_info(mut self, wallet_address: String, email: String, status: i32) -> Self {
        self.wallet_address = Some(wallet_address);
        self.email = Some(email);
        self.status = Some(status);
        self.action = Some(TopicReadActionType::UserInfo);
        self
    }
    pub fn check_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self.action = Some(TopicReadActionType::CheckEmail);
        self
    }

    pub fn no_param_action(mut self) -> Self {
        self.action = Some(TopicReadActionType::NoParamAction);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicReadActionType {
    UserInfo,
    CheckEmail,
}
impl TopicClient {
    pub async fn no_param_action(
        &self,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().no_param_action();
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn user_info(
        &self,
        wallet_address: String,
        email: String,
        status: i32,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().user_info(wallet_address, email, status);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn check_email(&self, email: String) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().check_email(email);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn query(&self, params: TopicQuery) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn get(&self, id: &str) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::get(&endpoint).await
    }
    pub async fn search_by(
        &self,
        size: usize,
        bookmark: Option<String>,
        description: String,
        created_at: i64,
    ) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicQuery {
            size,
            bookmark,
            action: Some(TopicQueryActionType::SearchBy),
            description: Some(description),
            created_at: Some(created_at),
            ..TopicQuery::default()
        };
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn date_from(
        &self,
        size: usize,
        bookmark: Option<String>,
        created_at: i64,
    ) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicQuery {
            size,
            bookmark,
            action: Some(TopicQueryActionType::DateFrom),
            created_at: Some(created_at),
            ..TopicQuery::default()
        };
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn act(&self, params: TopicAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        rest_api::post(&endpoint, params).await
    }
    pub async fn create(&self, title: String, description: String) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let req = TopicAction::Create(TopicCreateRequest { title, description });
        rest_api::post(&endpoint, req).await
    }
    pub async fn act_by_id(&self, id: &str, params: TopicByIdAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::post(&endpoint, params).await
    }
    pub async fn update(
        &self,
        id: &str,
        description: String,
        status: i32,
        tags: Vec<String>,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        let req = TopicByIdAction::Update(TopicUpdateRequest {
            description,
            status,
            tags,
        });
        rest_api::post(&endpoint, req).await
    }
}
```

### Define a model with parent ID
- If a resource is based on a specific parent resource, you can specify a parent ID with `:`.
  - Usually, the ID should be kebab-case by REST API convention.

``` rust
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/topics/v1/:topic-id/comments", iter_type=Vec)]
pub struct Comment {
    pub id: String,
    #[api_model(action = comment, related = String, read_action = search_by)]
    pub content: String,
    #[api_model(action_by_id = update, related = i64)]
    pub updated_at: i64,
}
```

#### Expanded code

``` rust
pub struct Comment {
    pub id: String,
    pub content: String,
    pub updated_at: i64,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentAction {
    Comment(String),
}
impl CommentClient {
    pub async fn act(&self, topic_id: &str, params: CommentAction) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        rest_api::post(&endpoint, params).await
    }
    pub async fn comment(&self, topic_id: &str, request: String) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        rest_api::post(&endpoint, request).await
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentByIdAction {
    Update(i64),
}
impl CommentClient {
    pub async fn act_by_id(
        &self,
        topic_id: &str,
        id: &str,
        params: CommentByIdAction,
    ) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::post(&endpoint, params).await
    }
    pub async fn update(&self, topic_id: &str, id: &str, request: i64) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::post(&endpoint, request).await
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentSummary {}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentQuery {
    pub size: usize,
    pub bookmark: Option<String>,
}
impl CommentQuery {
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
}
impl CommentClient {}
impl Comment {
    pub fn get_client(endpoint: &str) -> CommentClient {
        CommentClient {
            endpoint: endpoint.to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct CommentClient {
    pub endpoint: String,
}
impl CommentClient {
    pub async fn query(
        &self,
        topic_id: &str,
        params: CommentQuery,
    ) -> crate::Result<Vec<CommentSummary>> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn get(&self, topic_id: &str, id: &str) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::get(&endpoint).await
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentReadAction {
    pub action: Option<CommentReadActionType>,
    pub content: Option<String>,
}
impl CommentReadAction {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn search_by(mut self, content: String) -> Self {
        self.content = Some(content);
        self.action = Some(CommentReadActionType::SearchBy);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentReadActionType {
    SearchBy,
}
impl CommentClient {
    pub async fn search_by(&self, topic_id: &str, content: String) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        let params = CommentReadAction::new().search_by(content);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
}

```
