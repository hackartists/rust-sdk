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

    #[api_model(base = "/", table = nested_join_models)]
    pub struct NestedJoinModel {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        pub name: String,
        #[api_model(one_to_many = nested_child_models, foreign_key = parent_id)]
        pub children: Vec<NestedChildModel>,
    }

    #[api_model(base = "/", table = nested_child_models)]
    pub struct NestedChildModel {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        pub name: String,

        #[api_model(many_to_one = nested_join_models)]
        pub parent_id: i64,

        #[api_model(one_to_many = nested_joined_child_models, foreign_key = child_id)]
        pub joined_children: Vec<NestedJoinedChildModel>,

        #[api_model(many_to_many = nested_joined_child_many_models_inters, foreign_table_name = nested_joined_child_many_models, foreign_reference_key = child_id, foreign_primary_key = many_id)]
        pub joined_children_many: Vec<NestedJoinedChildManyModel>,
    }

    #[api_model(base = "/", table = nested_joined_child_models)]
    pub struct NestedJoinedChildModel {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        pub child: String,

        #[api_model(many_to_one = nested_child_models)]
        pub child_id: i64,
    }

    #[api_model(base = "/", table = nested_joined_child_many_models)]
    pub struct NestedJoinedChildManyModel {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        pub many: String,
    }

    #[api_model(base = "/", table = nested_joined_child_many_models_inters)]
    pub struct NestedJoinedChiuldManyModelsInter {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_one = nested_child_models)]
        pub child_id: i64,

        #[api_model(many_to_one = nested_joined_child_many_models)]
        pub many_id: i64,
    }

    #[tokio::test]
    async fn test_nested_join_query() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init();

        // let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        //     .max_connections(5)
        //     .connect(
        //         option_env!("DATABASE_URL")
        //             .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
        //     )
        //     .await
        //     .unwrap();
        // let now = SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();

        // let name = format!("nested-joined-{}", now);

        // let l1 = NestedJoinModel::get_repository(pool.clone());
        // let l2 = NestedChildModel::get_repository(pool.clone());
        // let l3 = NestedJoinedChildModel::get_repository(pool.clone());
        // let l4 = NestedJoinedChildManyModel::get_repository(pool.clone());
        // let l5 = NestedJoinedChiuldManyModelsInter::get_repository(pool.clone());

        // l1.create_this_table().await;
        // l2.create_this_table().await;
        // l3.create_this_table().await;
        // l4.create_this_table().await;
        // l5.create_this_table().await;

        // l1.create_table().await;
        // l2.create_table().await;
        // l3.create_table().await;
        // l4.create_table().await;
        // l5.create_table().await;

        // let doc1 = l1.insert(name.clone()).await.unwrap();
        // let doc2 = l2.insert(format!("{name}-child"), doc1.id).await.unwrap();
        // let doc3 = l3
        //     .insert(format!("{name}-child-1-to-n"), doc2.id)
        //     .await
        //     .unwrap();
        // let doc4 = l4.insert(format!("{name}-child-n-to-n")).await.unwrap();
        // tracing::info!("doc2: {:?} doc4: {:?}", doc2, doc4);
        // let doc5 = l5.insert(doc2.id, doc4.id).await.unwrap();

        // let got = NestedChildModel::query_builder()
        //     .id_equals(doc2.id)
        //     .query()
        //     .map(NestedChildModel::from)
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();

        // assert_eq!(got.id, doc2.id);
        // assert_eq!(got.name, format!("{name}-child"));
        // assert_eq!(got.parent_id, doc1.id);
        // assert_eq!(got.joined_children.len(), 1);
        // assert_eq!(got.joined_children[0].id, doc3.id);
        // assert_eq!(got.joined_children[0].child, format!("{name}-child-1-to-n"));
        // assert_eq!(got.joined_children[0].child_id, doc2.id);
        // assert_eq!(got.joined_children_many.len(), 1);
        // assert_eq!(got.joined_children_many[0].id, doc4.id);
        // assert_eq!(
        //     got.joined_children_many[0].many,
        //     format!("{name}-child-n-to-n")
        // );

        // let got = NestedJoinModel::query_builder()
        //     .id_equals(doc1.id)
        //     .query()
        //     .map(NestedJoinModel::from)
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();

        // assert_eq!(got.id, doc1.id);
        // assert_eq!(got.name, name);
        // assert_eq!(got.children.len(), 1);
        // assert_eq!(got.children[0].id, doc2.id);
        // assert_eq!(got.children[0].name, format!("{name}-child"));
        // assert_eq!(got.children[0].parent_id, doc1.id);
        // assert_eq!(got.children[0].joined_children.len(), 1);
        // assert_eq!(got.children[0].joined_children[0].id, doc3.id);
        // assert_eq!(
        //     got.children[0].joined_children[0].child,
        //     format!("{name}-child-1-to-n")
        // );
        // assert_eq!(got.children[0].joined_children[0].child_id, doc2.id);
        // assert_eq!(got.children[0].joined_children_many.len(), 1);
        // assert_eq!(got.children[0].joined_children_many[0].id, doc4.id);
        // assert_eq!(
        //     got.children[0].joined_children_many[0].many,
        //     format!("{name}-child-n-to-n")
        // );
    }
}
