use crate::common_types::{CreatedAt, UpdatedAt};
use crate::models::user_model::UserId;
use crate::utils::password_util::HashedPassword;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserCredentialModel {
    pub user_id: UserId,
    pub hashed_password: HashedPassword,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}