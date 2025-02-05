#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
mod empty_param_tests {
    #![allow(unused)]
    use super::*;
    use std::time::SystemTime;

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
    #[derive(validator::Validate)]
    #[api_model(base = "/panels/v2", table = panels, iter_type=QueryResponse)]
    pub struct PanelV2 {
        #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
        pub id: String,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(summary, action = [create], action_by_id = update)]
        pub name: String,
        #[api_model(summary, action = [create], action_by_id = update)]
        pub user_count: u64,
    }
}
