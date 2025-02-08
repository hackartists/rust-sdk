#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
#[cfg(test)]
pub mod update_into_tests {
    #![allow(unused)]
    use super::*;
    use std::{fmt::Arguments, time::SystemTime};

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::{postgres::PgRow, Execute};

    #[api_model(base = "/models", table = query_builder_test)]
    pub struct QueryModel {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, query_action = search)]
        pub name: String,

        #[api_model(query_action = search)]
        pub description: String,

        #[api_model(query_action = list_by_status)]
        pub status: i32,

        #[api_model(summary)]
        pub num: i64,

        pub is_like: bool,
    }

    #[tokio::test]
    async fn test_query_builder() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init();

        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let name = format!("name-{}", now);
        let description = format!("desc-{}", now);
        let status = 1;

        let repo = QueryModel::get_repository(pool.clone());
        repo.create_table().await;

        for b in [true, false].iter() {
            for n in 0..10 {
                repo.insert(
                    format!("{} {}-{}", name, n, b),
                    description.clone(),
                    status,
                    n,
                    *b,
                )
                .await
                .unwrap();
            }
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 20);

        let mut q = QueryModel::query_builder()
            .num_between(0, 5)
            .name_contains(name.clone())
            .order_by_id_asc();

        assert_eq!(
            q.sql(),
            "SELECT * FROM query_builder_test WHERE num BETWEEN $1 AND $2 AND name ILIKE $3 ORDER BY id ASC"
        );

        let mut q = q.with_count();

        assert_eq!(
            q.sql(),
            "SELECT COUNT(*) OVER() as total_count, * FROM query_builder_test WHERE num BETWEEN $1 AND $2 AND name ILIKE $3 ORDER BY id ASC"
        );

        let sq = QueryModelSummary::query_builder().sql();
        assert_eq!(
            sq,
            "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name, num FROM query_builder_test "
        );

        let doc: QueryModel = q
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(doc.name, format!("{} 0-true", name));

        let docs: Vec<QueryModel> = q
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 12);
        let mut total: i64 = 0;

        let docs: Vec<QueryModel> = q
            .clone()
            .limit(3)
            .query()
            .map(|r: PgRow| {
                use sqlx::Row;
                total = r.get("total_count");
                r.into()
            })
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 3);
        assert_eq!(total, 12);
        assert_eq!(docs[0].name, format!("{} 0-true", name));
        assert_eq!(docs[1].name, format!("{} 1-true", name));
        assert_eq!(docs[2].name, format!("{} 2-true", name));

        let docs: Vec<QueryModel> = q
            .clone()
            .limit(6)
            .page(2)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 6);
        assert_eq!(docs[0].name, format!("{} 0-false", name));
        assert_eq!(docs[1].name, format!("{} 1-false", name));
        assert_eq!(docs[2].name, format!("{} 2-false", name));
    }
}
