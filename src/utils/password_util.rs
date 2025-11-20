use bcrypt::{hash, verify};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error, Deserialize)]
pub enum HashedPasswordGenerationError {
    #[error("empty password")]
    EmptyPassword,
    #[error("hashing failed: {0}")]
    HashingFailed(String),
}

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(value: String) -> Self {
        PlainPassword {
            0: value
        }
    }
}

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct HashedPassword(String);

impl TryFrom<PlainPassword> for HashedPassword {
    type Error = HashedPasswordGenerationError;

    fn try_from(value: PlainPassword) -> Result<Self, Self::Error> {
        // if the plain text password is empty.
        let trimmed_password = value.0.trim();
        if trimmed_password.is_empty() {
            return Err(HashedPasswordGenerationError::EmptyPassword);
        }

        let hashed_password = hash(value.0, 8);
        match hashed_password {
            Ok(hp) => Ok(HashedPassword(hp)),
            Err(e) => Err(HashedPasswordGenerationError::HashingFailed(e.to_string())),
        }
    }
}

impl HashedPassword {
    pub fn verify(&self, plain_password: &PlainPassword) -> bool {
        verify(&plain_password.0, &self.0).unwrap_or(false)
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let plain_password = PlainPassword::new("mysecretpassword".to_string());
        let hashed_password = HashedPassword::try_from(plain_password)
            .expect("Hashed password is expected");

        let plain_password_to_verify = PlainPassword::new("mysecretpassword".to_string());
        assert!(hashed_password.verify(&plain_password_to_verify));

        let wrong_password = PlainPassword::new("wrongpassword".to_string());
        assert!(!hashed_password.verify(&wrong_password));
    }

    #[test]
    fn empty_password_test() {
        let plain_password = PlainPassword::new("".to_string());
        let hashed_password = HashedPassword::try_from(plain_password);
        assert!(hashed_password.is_err());
    }
}