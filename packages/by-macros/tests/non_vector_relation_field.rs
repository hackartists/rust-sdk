#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
pub mod test_non_vector_relation_field {
    #![allow(unused)]
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::postgres::PgRow;
    use sqlx::Row;

    #[api_model(base = "/v1/bills", table = test_support_custom_type_bills)]
    pub struct Bill {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,

        #[api_model(summary)]
        pub bill_id: String,
        #[api_model(summary)]
        pub title: String,

        #[api_model(many_to_many = test_support_custom_type_votes, type = JSONB, foreign_table_name = test_support_custom_type_users, foreign_primary_key = user_id, foreign_reference_key = bill_id, unique)]
        pub user_vote: SupportVote,
    }

    #[api_model(base = "/v1/votes", table = test_support_custom_type_votes)]
    pub struct SupportVote {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(many_to_one = test_support_custom_type_users)]
        pub user_id: i64,
        #[api_model(many_to_one = test_support_custom_type_bills)]
        pub bill_id: i64,
    }

    #[api_model(base = "/v1/users", table = test_support_custom_type_users)]
    pub struct SupportUser {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action = signup)]
        pub email: String,
    }

    #[tokio::test]
    async fn test_support() {
        let _ = tracing_subscriber::fmt::try_init();
        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/support"),
            )
            .await
            .unwrap();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let u = SupportUser::get_repository(pool.clone());
        let v = SupportVote::get_repository(pool.clone());
        let b = Bill::get_repository(pool.clone());

        b.create_this_table().await;
        u.create_this_table().await;
        v.create_this_table().await;

        b.create_related_tables().await;
        u.create_related_tables().await;
        v.create_related_tables().await;

        let user = u.insert("test@gmail.com".to_string()).await;
        assert!(user.is_ok(), "{:?}", user);
        let user = user.unwrap();
        let bill = b.insert("SOME ID".to_string(), "title".to_string()).await;
        assert!(bill.is_ok(), "{:?}", bill);
        let bill = bill.unwrap();

        let r: std::result::Result<Bill, sqlx::Error> = Bill::query_builder()
            .id_equals(bill.id)
            .query()
            .map(Bill::from)
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok(), "{:?}", r);

        assert_eq!(r.unwrap().user_vote.id, 0);

        let vote = v.insert(user.id, bill.id).await;
        assert!(vote.is_ok(), "{:?}", vote);
        let vote = vote.unwrap();

        let r: std::result::Result<Bill, sqlx::Error> = Bill::query_builder()
            .id_equals(bill.id)
            .query()
            .map(Bill::from)
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok(), "{:?}", r);

        assert_ne!(r.unwrap().user_vote.id, 0);
    }
}
