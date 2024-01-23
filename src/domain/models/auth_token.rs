use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    pub username: String,
    pub roles: Option<Roles>,
    pub exp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Roles(Vec<String>);

impl AuthToken {
    pub fn without_roles(username: String) -> AuthToken {
        let expiration_time = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();
        AuthToken {
            username,
            roles: None,
            exp: expiration_time,
        }
    }
}
