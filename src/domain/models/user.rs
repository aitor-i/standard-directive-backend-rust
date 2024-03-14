#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub code: String,
}
