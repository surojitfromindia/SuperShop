use crate::common_types::{CreatedAt, UpdatedAt};
use crate::models::user_model::UserId;

#[derive(Debug, Clone)]
pub struct UserCredentialModel {
    pub user_id: UserId,
    pub hash_password: String,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}