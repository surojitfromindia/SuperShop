use crate::common_types::{DatabaseError, PublicId};
use crate::models::user_model::{UserId, UserModel};
use async_trait::async_trait;
use serde::Deserialize;
use crate::models::user_credential_model::UserCredentialModel;

// what should be the name of this struct?
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub hashed_password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreatedUser {
    pub public_id: PublicId,
    pub email: String,
}

#[async_trait]
pub trait  UserRepositoryTrait: Send + Sync {
    async fn create_user(&self, user: NewUser) -> anyhow::Result<CreatedUser, DatabaseError>;
    async fn get_user_by_id(&self, user_id: &UserId)-> anyhow::Result<Option<UserModel>,DatabaseError>;
    async fn get_user_by_public_id(&self, public_id: &PublicId)-> anyhow::Result<Option<UserModel>,DatabaseError>;
    async fn get_user_by_email(&self, email : &String)-> anyhow::Result<Option<UserModel>,DatabaseError>;

    async fn get_user_credentials_by_id(&self, use_id: &UserId) -> anyhow::Result<Option<UserCredentialModel>, DatabaseError>;
}