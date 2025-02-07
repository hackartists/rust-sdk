use std::collections::HashMap;

use axum::{
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use reqwest::header::{self, IntoHeaderName};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Default, aide::OperationIo)]
pub struct JsonWithHeaders<T> {
    pub body: T,
    pub headers: HeaderMap,
    pub cookies: HashMap<String, String>,
}

impl<T> JsonWithHeaders<T> {
    pub fn new(body: T) -> Self {
        Self {
            body,
            headers: HeaderMap::new(),
            cookies: HashMap::new(),
        }
    }

    pub fn insert_header<K>(mut self, key: K, value: &str) -> Self
    where
        K: IntoHeaderName,
    {
        self.headers
            .insert(key, HeaderValue::from_str(&value).unwrap());
        self
    }

    pub fn with_cookie(mut self, key: &str, value: &str) -> Self {
        self.cookies.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_authorization(mut self, auth_type: &str, value: &str) -> Self {
        let authz = format!("{} {}", auth_type, value);
        self.headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&authz).unwrap(),
        );

        self.with_cookie("token", value)
            .with_cookie("type", auth_type)
    }

    pub fn with_bearer_token(self, token: &str) -> Self {
        self.with_authorization("Bearer", token)
    }
}

impl<T> serde::Serialize for JsonWithHeaders<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.body.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for JsonWithHeaders<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let body = T::deserialize(deserializer)?;
        Ok(JsonWithHeaders {
            body,
            headers: HeaderMap::new(),
            cookies: HashMap::new(),
        })
    }
}

impl<T> schemars::JsonSchema for JsonWithHeaders<T>
where
    T: schemars::JsonSchema,
{
    fn schema_name() -> String {
        T::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        T::json_schema(gen)
    }
}

impl<T> IntoResponse for JsonWithHeaders<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        let expires_time = Utc::now() + Duration::hours(24);
        let exp = expires_time.format("%a, %d %b %Y %T GMT").to_string();

        let mut headers = self.headers.clone();
        for (k, v) in self.cookies {
            headers.append(
                header::SET_COOKIE,
                format!(
                    "{}={}; Path=/; HttpOnly; Max-Age=86400; Expires={}; SameSite={}",
                    k,
                    v,
                    exp,
                    match option_env!("ENV") {
                        Some("local") => "None".to_string(),
                        Some(_) => format!(
                            "None; Secure; Domain={}",
                            option_env!("BASE_DOMAIN").expect("BASE_DOMAIN is not set")
                        ),
                        None => panic!("ENV is not set"),
                    }
                )
                .parse()
                .unwrap(),
            );
        }

        (axum::http::StatusCode::OK, headers, axum::Json(self.body)).into_response()
    }
}
