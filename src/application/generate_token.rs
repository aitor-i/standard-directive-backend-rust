use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};

use crate::domain::models::auth_token::AuthToken;

pub fn generate_token(user_token: AuthToken) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret("test_secret".as_ref());

    encode(&Header::default(), &user_token, &encoding_key)
}
