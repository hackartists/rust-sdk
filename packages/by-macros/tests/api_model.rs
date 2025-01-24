#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;

use by_macros::api_model;

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(not(feature = "server"))]
#[derive(Eq, PartialEq)]
#[api_model(base = "/topics/v1", iter_type=Vec, table = topics)] // rename is supported but usually use default(snake_case)
pub struct Topic {
    #[api_model(summary, primary_key)]
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
    #[api_model(summary, query_action = [search_by, date_from], auto  = insert)]
    pub created_at: i64,
    #[api_model(many_to_many = topic_user_likes, foreign_table_name = users, foreign_key = id, foreign_key_type = TEXT)]
    pub is_liked: bool,

    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action_by_id = like, related = CommentRequest)]
    #[api_model(action = comment, related = Comment)]
    #[api_model(one_to_many = comments, foreign_key = topic_id)]
    pub comments: Vec<Comment>,

    #[api_model(action_by_id = update)]
    pub tags: Vec<String>,
}

#[cfg(not(feature = "server"))]
#[derive(Eq, PartialEq)]
#[api_model(base = "/topics/v1/:topic-id/comments", iter_type=Vec)]
pub struct Comment {
    pub id: String,
    #[api_model(action = comment, related = String, read_action = search_by)]
    pub content: String,
    #[api_model(action_by_id = update, related = i64)]
    pub updated_at: i64,

    #[api_model(many_to_one = topics, foreign_key = id, foreign_key_type = TEXT)]
    pub topic_id: String,
}

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentRequest {
    pub comment_id: String,
    pub is_liked: bool,
}

#[cfg(not(feature = "server"))]
mod normal {
    use super::*;

    #[test]
    fn test_macro_expansion_topic() {
        let _read_action = TopicReadAction {
            action: Some(TopicReadActionType::CheckEmail),
            wallet_address: None,
            email: Some("test".to_string()),
            status: Some(1),
        };

        let _ = TopicReadAction::new().user_info("wallet".to_string(), "email".to_string(), 1);
        let _ = TopicReadAction::new().check_email("email".to_string());

        let q = TopicQuery {
            action: Some(TopicQueryActionType::DateFrom),
            size: 10,
            bookmark: None,
            description: None,
            status: Some(1),
            created_at: None,
        };

        assert_eq!(q.status, Some(1));
        assert_eq!(q.size, 10);
        assert_eq!(q.bookmark, None);
        assert_eq!(q.description, None);

        let _q = TopicQuery::new(10)
            .with_bookmark("test".to_string())
            .search_by("test".to_string(), 0);

        let q = TopicQuery::new(10)
            .with_bookmark("test".to_string())
            .date_from(1);

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

        let create_request = TopicAction::Create(create_request);
        let update_request = TopicByIdAction::Update(update_request);
        let like_request = TopicByIdAction::Like(CommentRequest {
            comment_id: "1".to_string(),
            is_liked: true,
        });
        let comment_request = TopicAction::Comment(Comment {
            id: "1".to_string(),
            content: "content".to_string(),
            updated_at: 0,
            topic_id: "1".to_string(),
        });

        let cli = Topic::get_client("");
        let _ = cli.get("1");
        let _ = cli.query(q);
        let _ = cli.act(create_request);
        let _ = cli.act(comment_request);
        let _ = cli.act_by_id("1", update_request);
        let _ = cli.act_by_id("1", like_request);
        let _ = cli.user_info("wallet".to_string(), "email".to_string(), 1);
        let _ = cli.check_email("email".to_string());
        // let _ = cli.search_by("test".to_string(), 0);
        let _ = cli.create("title".to_string(), "description".to_string());
        let _ = cli.update("id", "description".to_string(), 1, vec!["tag".to_string()]);
        let _ = cli.search_by(
            1,
            Some("bookmark".to_string()),
            "description".to_string(),
            10,
        );
        let _ = cli.date_from(10, None, 20240101);
    }

    #[test]
    fn test_macro_expansion_comment() {
        let cli = Comment::get_client("");
        let _ = cli.get("topic-id", "comment-id");
        let _ = cli.query("topic-id", CommentQuery::new(10));
        let _ = cli.act("topic-id", CommentAction::Comment("content".to_string()));
        let _ = cli.comment("topic-id", "content".to_string());
        let _ = cli.search_by("topic-id", "content".to_string());
        let _ = cli.update("topic-id", "comment-id", 100);
        let _ = cli.act_by_id("topic-id", "comment-id", CommentByIdAction::Update(100));
    }
}

#[cfg(feature = "server")]
mod server_tests {
    use std::time::SystemTime;
    use validator::Validate;

    use super::*;

    #[derive(validator::Validate)]
    #[api_model(base = "/users/v1", read_action = user_info, table = users, iter_type=Vec)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: String,
        #[api_model(auto = insert)]
        pub created_at: u64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(action = signup)]
        pub nickname: String,
        #[api_model(unique, read_action=check_principal)]
        pub principal: String,
        #[api_model(action = signup, read_action = check_email, unique, queryable, action_by_id = update)]
        #[validate(email)]
        pub email: String,
        #[api_model(action = signup)]
        pub profile_url: String,
    }

    #[test]
    fn test_macro_expansion_user() {
        let _ = User {
            id: "id".to_string(),
            created_at: 0,
            updated_at: 0,
            principal: "principal".to_string(),
            nickname: "nickname".to_string(),
            email: "email".to_string(),
            profile_url: "profile_url".to_string(),
        };

        let mut read = UserReadAction {
            action: Some(UserReadActionType::CheckEmail),
            email: Some("email".to_string()),
            principal: Some("principal".to_string()),
        };

        assert!(read.validate().is_err());

        read.email = Some("abc@test.com".to_string());
        assert!(read.validate().is_ok());

        let mut q = UserQuery {
            size: 10,
            bookmark: None,
            email: Some("email".to_string()),
        };
        assert!(q.validate().is_err());

        q.email = Some("abc@test.com".to_string());
        assert!(q.validate().is_ok());

        let mut action = UserSignupRequest {
            nickname: "nickname".to_string(),
            email: "email".to_string(),
            profile_url: "profile_url".to_string(),
        };
        assert!(action.validate().is_err());

        action.email = "abc@test.com".to_string();
        assert!(action.validate().is_ok());

        assert!(UserAction::Signup(action).validate().is_ok());

        let mut action = UserUpdateRequest {
            email: "email".to_string(),
        };
        assert!(action.validate().is_err());

        action.email = "abc@test.com".to_string();
        assert!(action.validate().is_ok());

        assert!(UserByIdAction::Update(action).validate().is_ok());

        let cli = User::get_client("");
        let _ = cli.user_info();
        let _ = cli.check_email("email".to_string());
    }

    #[tokio::test]
    async fn test_db_create() {
        use sqlx::{postgres::PgPoolOptions, Postgres};
        let _ = tracing_subscriber::fmt::try_init();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let email = format!("test-{}@test.com", now);
        let principal = format!("{}-principal", now);

        let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/test"),
            )
            .await
            .unwrap();

        let repo = User::get_repository(pool.clone());
        repo.create_table().await.unwrap();
        repo.insert(
            "nickname".to_string(),
            principal.clone(),
            email.clone(),
            "profile_url".to_string(),
        )
        .await
        .unwrap();

        match repo
            .insert(
                "nickname".to_string(),
                principal.clone(),
                email.clone(),
                "profile_url".to_string(),
            )
            .await
        {
            Ok(_) => assert!(false, "Unique constraint violation"),
            Err(e) => {
                println!("{}", e);
            }
        };

        let user = match repo
            .find_one(&UserReadAction::new().check_email(email.clone()))
            .await
        {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };

        assert_eq!(user.principal, principal);
        assert_eq!(user.email, email);

        let (users_1, total) = match repo.find(&UserQuery::new(5).with_page(1)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };
        assert_eq!(users_1.len(), 5);
        assert_eq!(total > 0, true);

        let (users_2, total2) = match repo.find(&UserQuery::new(5).with_page(1)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };
        assert_eq!(users_2.len(), 5);
        assert_eq!(total2, total);

        for i in 0..users_1.len() {
            assert_eq!(users_1[i].principal, users_2[i].principal);
            assert_eq!(users_1[i].email, users_2[i].email);
        }

        let (users_3, total3) = match repo.find(&UserQuery::new(5).with_page(2)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };
        assert_eq!(total3, total);

        for i in 0..users_1.len() {
            assert_ne!(users_1[i].principal, users_3[i].principal);
            assert_ne!(users_1[i].email, users_3[i].email);
        }

        let (users_4, total4) = match repo.find(&UserQuery::new(2).with_page(2)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };

        assert_eq!(total4, total);
        assert_eq!(users_4.len(), 2);

        for i in 0..users_4.len() {
            assert_ne!(users_3[i].principal, users_4[i].principal);
            assert_ne!(users_3[i].email, users_4[i].email);
        }
    }
}
