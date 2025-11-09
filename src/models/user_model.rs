use crate::common_types::{CreatedAt, PrimaryId, PublicId, UpdatedAt};

#[allow(dead_code)]
pub type UserId = PrimaryId;

#[derive(Debug,Clone,sqlx::Type )]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Deleted,
    Suspended,
}
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserModel {
    pub id: UserId,
    pub public_id: PublicId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub status: UserStatus,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}