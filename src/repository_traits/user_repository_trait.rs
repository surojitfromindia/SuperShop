use crate::common_types::{DatabaseError, PublicId};
use crate::models::user_model::UserModel;
use async_trait::async_trait;
use serde::Deserialize;


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
    async fn get_user_by_id(&self,)-> anyhow::Result<UserModel,DatabaseError>;
    async fn get_user_by_public_id(&self,)-> anyhow::Result<UserModel,DatabaseError>;
    async fn get_user_by_email(&self, email : &String)-> anyhow::Result<Option<UserModel>,DatabaseError>;
}