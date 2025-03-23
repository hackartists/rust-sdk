use candid::CandidType;
pub use ic_cdk::*;
pub use ic_cdk_timers::*;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub upgrade: Option<bool>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn build<T>(body: T) -> Self
    where
        T: serde::Serialize,
    {
        let body = serde_json::to_vec(&body).unwrap();

        Response {
            status_code: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
            body,
            upgrade: None,
        }
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_upgrade(mut self) -> Self {
        self.upgrade = Some(true);
        self
    }

    pub fn upgrade() -> Self {
        Response {
            upgrade: Some(true),
            ..Default::default()
        }
    }
}
