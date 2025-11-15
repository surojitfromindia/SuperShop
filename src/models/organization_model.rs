use crate::common_types::{CreatedAt, OrganizationId, PublicId, UpdatedAt, UserId};


#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "organization_status", rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Deleted,
    Suspended,
}
#[derive(Debug, Clone)]
pub struct OrganizationModel {
    pub id: OrganizationId,
    pub public_id: PublicId,

    pub name: String,
    pub name_sl: Option<String>,

    pub status: OrganizationStatus,

    pub created_by_user_id: UserId,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}