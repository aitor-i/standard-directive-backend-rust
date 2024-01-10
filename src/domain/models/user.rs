#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    user_name: String,
    password: String,
    user_id: UserId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct UserId(String);
