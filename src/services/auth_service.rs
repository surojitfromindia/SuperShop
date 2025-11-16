use crate::common_types::{DatabaseError, UserId};
use crate::models::user_model::UserModel;
use crate::repositories::user_repository::UserRepository;
use crate::repository_traits::user_repository_trait::UserRepositoryTrait;
use crate::types::{PlainPassword, PublicId};
use crate::utils::token::{Token, TokenGenerationError, TokenType};
use crate::AppState;
use thiserror::Error;

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
    DBError(DatabaseError),
    #[error("unknown error")]
    TokenError(TokenGenerationError),
}

#[derive(Debug)]
pub struct UserAccessContext {
    pub email: String,
    pub id: UserId,
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
            shop_db: self.app_state.shop_db.clone(),
        };

        // find user by email
        let user = user_repository
            .get_user_by_email(&user_login_input.email)
            .await
            .map_err(|e| UserLoginError::DBError(e))?;
        if user.is_none() {
            return Err(UserLoginError::InvalidEmail);
        }

        let UserModel { id, public_id, .. } = user.unwrap();

        // find the credentials
        let user_credential = user_repository
            .get_user_credentials_by_id(&id)
            .await
            .map_err(|e| UserLoginError::DBError(e))?;

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
            .map_err(|e| UserLoginError::TokenError(e))?;

        Ok(UserLoginOutput { token, public_id })
    }


    pub async fn get_user_for_access_context(&self, public_id: &PublicId) -> anyhow::Result<Option<UserAccessContext>> {
        let user_repository = UserRepository {
            shop_db: self.app_state.shop_db.clone(),
        };
        let user = user_repository
            .get_user_by_public_id(&public_id)
            .await?;
        Ok(user.map(|x| {
            UserAccessContext {
                email: x.email,
                id: x.id,
            }
        }))
    }
}
