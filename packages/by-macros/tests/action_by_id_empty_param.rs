#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

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

#[api_model(base = "/models", table = action_empty_params, iter_type=QueryResponse, action_by_id = delete, action = [no_param, empty])]
pub struct ActionEmptyParamModel {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
}

#[tokio::test]
async fn test_action_by_id_empty_param() {
    let cli = ActionEmptyParamModel::get_client("test");
    cli.delete("0");

    cli.no_param();
    cli.empty();

    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(
            option_env!("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
        )
        .await
        .unwrap();

    let repo = ActionEmptyParamModel::get_repository(pool);
    repo.delete("0".to_string());

    let req_by_id = ActionEmptyParamModelByIdAction::Delete(ActionEmptyParamModelDeleteRequest {});
}
