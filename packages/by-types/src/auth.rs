use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub role: Role,
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Default, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin = 0,
    #[default]
    User = 1,
    // It means the user is not signed in web page.
    Guest = 10,
}

impl TryFrom<i32> for Role {
    type Error = String;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::Admin),
            1 => Ok(Role::User),
            10 => Ok(Role::Guest),
            _ => Err(format!("Invalid Role: {}", value)),
        }
    }
}

impl Into<i32> for Role {
    fn into(self) -> i32 {
        self as i32
    }
}
