use crate::common_types::{DBTransaction, DatabaseError, OrganizationId, PublicIdCounter};
use crate::models::user_model::UserId;
use async_trait::async_trait;
use crate::types::PrimaryId;

pub struct NewBranch {
    pub name: String,
    pub name_sl: Option<String>,
    pub organization_id: OrganizationId,
    pub created_by_user_id: UserId,
    pub is_default: Option<bool>,
}

pub struct CreatedBranch {
    pub id: PrimaryId,
}

#[async_trait]
pub trait BranchRepositoryTrait: Send + Sync {
    async fn create_branch_public_id_counter(&self, tx: &mut DBTransaction, counter: PublicIdCounter<'_>) -> anyhow::Result<(), DatabaseError>;
    async fn create_branch(&self, tx: &mut DBTransaction<'_>, new_branch: NewBranch) -> anyhow::Result<CreatedBranch, DatabaseError>;
}