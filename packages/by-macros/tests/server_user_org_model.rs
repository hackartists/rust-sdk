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
use std::time::SystemTime;
use validator::Validate;

#[cfg(feature = "server")]
mod server_tests {
    use super::*;

    #[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], table = uo_users, iter_type=QueryResponse)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: String,
        #[api_model(auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(action = [signup, login, reset], unique, read_action = get_user)]
        pub email: String,
        #[api_model(action = [signup, login, reset], read_action = get_user)]
        pub password: String,
    }

    #[api_model(base = "/auth/v1/organizations", table = uo_organizations, iter_type=QueryResponse)]
    pub struct Organization {
        #[api_model(summary, primary_key)]
        pub id: String,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary)]
        pub name: String,
        #[api_model(many_to_many = uo_user_orgs, foreign_table_name = uo_users, foreign_primary_key = user_id, foreign_reference_key = org_id)]
        pub users: Vec<User>,
    }
}

mod created {
    use super::*;
}
