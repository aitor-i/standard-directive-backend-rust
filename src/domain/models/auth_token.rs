use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    pub username: String,
    pub roles: Option<Roles>,
    pub exp: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Roles(Vec<String>);

impl AuthToken {
    pub fn without_roles(username: String) -> AuthToken {
        AuthToken {
            username,
            roles: None,
            exp: 10000000000,
        }
    }
}
