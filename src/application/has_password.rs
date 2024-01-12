use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
