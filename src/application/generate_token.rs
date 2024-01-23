use dotenv::dotenv;
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use std::env;

use crate::domain::models::auth_token::AuthToken;

pub fn generate_token(user_token: AuthToken) -> Result<String, Error> {
    dotenv().ok();

    let secret = env::var("LOGIN_TOKEN_KEY").expect("Error geting token key");
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::default(), &user_token, &encoding_key)
}
