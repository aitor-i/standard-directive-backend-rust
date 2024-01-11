#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub username: String,
    password: String,
    user_id: UserId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserId(String);
