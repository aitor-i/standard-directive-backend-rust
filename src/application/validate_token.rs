use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::domain::models::auth_token::AuthToken;

pub fn validate_token(token: &str) -> Result<AuthToken, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret("test_secret".as_ref());

    let validation = Validation::default();

    decode::<AuthToken>(token, &decoding_key, &validation).map(|data| data.claims)
}
