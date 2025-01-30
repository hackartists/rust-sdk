#[cfg(feature = "server")]
use by_axum::aide;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
}

impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
    fn from((items, total_count): (Vec<T>, i64)) -> Self {
        QueryResponse { items, total_count }
    }
}

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
mod server_tests {
    use by_macros::api_model;
    use std::time::SystemTime;

    use super::*;

    #[api_model(base = "/topics/v1", table = test_topics, iter_type=QueryResponse)]
    pub struct Topic {
        #[api_model(summary, primary_key)]
        pub id: String,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action = create)]
        pub title: String,
        #[api_model(summary, action = create)]
        pub content: String,

        #[api_model(summary, action = create)]
        pub started_at: i64,
        // The end time of the vote
        #[api_model(summary, action = create)]
        pub ended_at: i64,
        // The number of required votes
        #[api_model(summary, action = create)]
        pub requirement: i64,
        // The number of voters
        // #[api_model(summary, skip)]
        // pub voters: i64,

        // // Donation amount
        // #[api_model(summary, skip)]
        // pub amount: i64,

        // #[api_model(summary, skip)]
        // pub voted: bool,
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
        let _principal = format!("{}-principal", now);

        let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:password@localhost:5432/test"),
            )
            .await
            .unwrap();

        let repo = Topic::get_repository(pool.clone());
        repo.create_table().await.unwrap();
    }
}
