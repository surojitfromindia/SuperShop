use std::time::Instant;
use crate::AppState;
use crate::models::user_model::UserModel;
use crate::repository_traits::user_repository_trait::UserRepositoryTrait;
use crate::types::{PlainPassword, PublicId};
use crate::utils::token::{Token, TokenType};
use thiserror::Error;
use crate::repositories::user_repository::UserRepository;

pub struct UserLoginInput {
    pub email: String,
    pub password: PlainPassword,
}

pub struct UserLoginOutput {
    pub public_id: PublicId,
    pub token: Token,
}

#[derive(Error, Debug)]
pub enum UserLoginError {
    #[error("wrong email or password")]
    InvalidEmail,
    #[error("wrong password")]
    InvalidPassword,
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("unknown error")]
    UnknownError,
}

pub struct AuthService {
    pub app_state: AppState,
}
impl AuthService {
    pub async fn user_login(
        &self,
        user_login_input: UserLoginInput,
    ) -> anyhow::Result<UserLoginOutput, UserLoginError> {

        let user_repository = UserRepository {
            shop_db : self.app_state.shop_db.clone(),
        };



        // find user by email
        let user = user_repository
            .get_user_by_email(&user_login_input.email)
            .await
            .map_err(|e| UserLoginError::DatabaseError(e.to_string()))?;
        if user.is_none() {
            return Err(UserLoginError::InvalidEmail);
        }

        let UserModel { id, public_id, .. } = user.unwrap();

        // find the credentials
        let user_credential = user_repository
            .get_user_credentials_by_id(&id)
            .await
            .map_err(|e| UserLoginError::DatabaseError(e.to_string()))?;

        // try to verify the password
        let hashed_password = user_credential.unwrap().hashed_password;

        if !hashed_password.verify(&user_login_input.password) {
            return Err(UserLoginError::InvalidPassword);
        }


        // generate token.
        let token = Token::new(
            &public_id,
            &TokenType::AccessToken,
            &self.app_state.env_config,
        )
        .map_err(|_| UserLoginError::UnknownError)?;

        Ok(UserLoginOutput { token, public_id })
    }
}
