use bcrypt::{hash, BcryptResult};

pub fn hash_password(clear_password: &str) -> BcryptResult<String> {
    hash(clear_password, 12)
}
