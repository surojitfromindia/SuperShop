use serde::Deserialize;
use crate::AppState;
use thiserror::Error;
use crate::repository_traits::user_repository_trait::{CreatedUser, NewUser, UserRepositoryTrait};
use crate::utils::password_util::{HashedPassword, HashedPasswordGenerationError, PlainPassword};

pub struct RegisterUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: PlainPassword,
    pub phone: Option<String>,
}
pub type RegisterUserOutput = CreatedUser;

#[derive(Deserialize,Debug,Error)]
#[non_exhaustive]
pub enum RegisterUserError {
    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("user with this email already exists: {0}")]
    DuplicateEmail(String),
    
    #[error("invalid user data: {0}")]
    ValidationError(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
    
    #[error("password error: {0}")]
    PasswordError(#[from] HashedPasswordGenerationError),

}

pub struct UserService {
    pub app_state: AppState,
}

// todo: some controller errors will be thrown from particular services;
impl UserService {
    pub async fn register_user(
        &self,
        user: RegisterUserInput,
    ) -> anyhow::Result<RegisterUserOutput, RegisterUserError> {
        let app_state = &self.app_state;

        let existing_user = app_state
            .repositories
            .user_repository
            .get_user_by_email(&user.email)
            .await
            .map_err(|e| RegisterUserError::DatabaseError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(RegisterUserError::DuplicateEmail(user.email));
        }

        let hash_password = HashedPassword::try_from(user.password)?;
        let new_user = NewUser {
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            hashed_password: hash_password.to_string(),
        };
        self.app_state
            .repositories
            .user_repository
            .create_user(new_user)
            .await
            .map_err(|e| RegisterUserError::DatabaseError(e.to_string()))
    }
}
