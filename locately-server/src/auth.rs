use bcrypt::{hash, BcryptResult};
use validator::ValidationError;

pub fn hash_password(clear_password: &str) -> BcryptResult<String> {
    hash(clear_password, 12)
}

pub fn is_password_matched(clear_password: &str, hashed_password: &str) -> bool {
    bcrypt::verify(clear_password, hashed_password).unwrap_or(false)
}

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let contains_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let contains_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let contains_digit = password.chars().any(|c| c.is_ascii_digit());

    if contains_uppercase && contains_lowercase && contains_digit && password.len() >= 8 {
        Ok(())
    } else {
        Err(ValidationError::new(
            "Password does not meet the required strength criteria.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_password_strength;

    #[test]
    fn test_validate_password_strength_weak_1() {
        let password = "password";
        let result = validate_password_strength(password);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_strength_weak_2() {
        let password = "PASSWORD";
        let result = validate_password_strength(password);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_strength_weak_3() {
        let password = "Password";
        let result = validate_password_strength(password);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_strength_short() {
        let password = "Pass1";
        let result = validate_password_strength(password);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_password_strength_strong() {
        let password = "Pass1word";
        let result = validate_password_strength(password);
        assert!(result.is_ok());
    }
}
