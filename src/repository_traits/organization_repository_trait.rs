use async_trait::async_trait;
use crate::common_types::{DBTransaction, DatabaseError, OrganizationId};
use crate::models::user_model::UserId;
use crate::types::PublicId;

pub struct NewOrganization {
    pub name: String,
    pub name_sl: Option<String>,
    pub created_by_user_id: UserId,
}

pub struct CreatedOrganization {
    pub public_id: PublicId,
    pub id: OrganizationId,
}

#[async_trait]
pub trait OrganizationRepositoryTrait: Send + Sync {
    async fn create_organization(&self, tx: &mut DBTransaction<'_>, new_organization: NewOrganization) -> anyhow::Result<CreatedOrganization, DatabaseError>;
}