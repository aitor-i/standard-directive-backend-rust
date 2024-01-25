use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: Option<String>,
    pub password: String,
    pub email: Option<String>,
}
