use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    username: String,
    roles: Option<Roles>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Roles(Vec<String>);

impl AuthToken {
    pub fn without_roles(username: String) -> AuthToken {
        AuthToken {
            username,
            roles: None,
        }
    }
}
