use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

use crate::domain::models::auth_token::AuthToken;

pub fn validate_token(token: &str) -> Result<AuthToken, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret = env::var("LOGIN_TOKEN_KEY").expect("Error geting token key");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    let validation = Validation::default();

    println!("Token validation, token: {}", token);

    decode::<AuthToken>(token, &decoding_key, &validation).map(|data| data.claims)
}
