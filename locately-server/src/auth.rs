use bcrypt::{hash, BcryptResult};

pub fn hash_password(clear_password: &str) -> BcryptResult<String> {
    hash(clear_password, 12)
}

pub fn is_password_matched(clear_password: &str, hashed_password: &str) -> bool {
    bcrypt::verify(clear_password, hashed_password).unwrap_or(false)
}
