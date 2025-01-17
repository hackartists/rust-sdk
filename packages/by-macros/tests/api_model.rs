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
    #[api_model(summary, queryable, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update)]
    pub status: i32,
    #[api_model(summary)]
    pub created_at: i64,
    pub is_liked: bool,

    pub updated_at: i64,
    #[api_model(action_by_id = like, related = CommentActionRequest)]
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
pub struct CommentActionRequest {
    pub comment_id: String,
    pub is_liked: bool,
}

#[test]
fn test_macro_expansion() {
    let q = TopicQuery {
        size: 10,
        bookmark: None,
        description: None,
        status: Some(1),
    };

    assert_eq!(q.status, Some(1));
    assert_eq!(q.size, 10);
    assert_eq!(q.bookmark, None);
    assert_eq!(q.description, None);

    let summary = TopicSummary::default();
    assert_eq!(summary.id, "".to_string());
    assert_eq!(summary.title, "".to_string());
    assert_eq!(summary.description, "".to_string());
    assert_eq!(summary.status, 0);
    assert_eq!(summary.created_at, 0);

    let create_request = TopicCreateRequest {
        title: "title".to_string(),
        description: "description".to_string(),
    };
    assert_eq!(create_request.title, "title".to_string());
    assert_eq!(create_request.description, "description".to_string());

    let update_request = TopicUpdateRequest {
        description: "description".to_string(),
        status: 1,
        tags: vec!["tag".to_string()],
    };
    assert_eq!(update_request.description, "description".to_string());
    assert_eq!(update_request.status, 1);
    assert_eq!(update_request.tags, vec!["tag".to_string()]);

    let create_request = TopicActionRequest::Create(create_request);
    let update_request = TopicActionByIdRequest::Update(update_request);
    let like_request = TopicActionByIdRequest::Like(CommentActionRequest {
        comment_id: "1".to_string(),
        is_liked: true,
    });
    let comment_request = TopicActionRequest::Comment(Comment {
        id: "1".to_string(),
        content: "content".to_string(),
    });

    let cli = Topic::get_client("");
    let _ = cli.get("1");
    let _ = cli.query(q);
    let _ = cli.act(create_request);
    let _ = cli.act(comment_request);
    let _ = cli.act_by_id("1", update_request);
    let _ = cli.act_by_id("1", like_request);
}
