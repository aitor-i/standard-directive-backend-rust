use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    username: String,
    roles: Roles,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Roles(Vec<String>);
