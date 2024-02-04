#[cfg(test)]
mod tests {
    use crate::{
        application::{generate_token::generate_token, validate_token::validate_token},
        domain::models::auth_token::AuthToken,
    };

    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use std::env;

    // Setup function to configure the environment for tests
    fn setup() {
        // Mock environment variable. Replace "your_secret_key_here" with a valid key for testing.
        env::set_var("LOGIN_TOKEN_KEY", "your_secret_key_here");
    }

    #[test]
    fn test_generate_token() {
        setup();

        // Create a sample AuthToken
        let auth_token = AuthToken::without_roles("test_user".to_string());

        // Attempt to generate a token
        let result = generate_token(auth_token.clone());
        assert!(result.is_ok(), "Token generation should succeed");

        // Further validation could include checking the structure of the token,
        // but that would require decoding the token here, which might be redundant.
    }

    #[test]
    fn test_validate_token() {
        setup();

        // Create a sample AuthToken and generate a token for it
        let auth_token = AuthToken::without_roles("test_user".to_string());
        let token = generate_token(auth_token).expect("Failed to generate token for testing");

        // Attempt to validate the token
        let validation_result = validate_token(&token);
        assert!(validation_result.is_ok(), "Token validation should succeed");

        let validated_token = validation_result.expect("Validation failed unexpectedly");
        assert_eq!(
            validated_token.username, "test_user",
            "The username should match the original"
        );

        // Optionally, further tests could assert roles and expiration logic
    }
}
