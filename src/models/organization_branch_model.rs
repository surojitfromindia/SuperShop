use crate::common_types::{CreatedAt, OrganizationId, PrimaryId, PublicId, UpdatedAt, UserId};

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "branch_status", rename_all = "snake_case")]
pub enum BranchStatus {
    Active,
    Deleted,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct OrganizationBranchModel {
    pub id: PrimaryId,
    pub public_id: PublicId,
    organization_id: OrganizationId,

    pub name: String,
    pub name_sl: Option<String>,

    pub status: BranchStatus,

    pub created_by_user_id: UserId,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
