use crate::common_types::{CreatedAt, PrimaryId, UpdatedAt};

#[allow(dead_code)]
pub type UserId = PrimaryId;

#[derive(Debug,Clone)]
pub enum UserStatus {
    Active,
    Deleted,
    Suspended,
}
#[derive(Debug, Clone)]
pub struct UserModel {
    pub id: UserId,
    pub public_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub status: UserStatus,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}