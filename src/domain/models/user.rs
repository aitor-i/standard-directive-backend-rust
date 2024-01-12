#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}
