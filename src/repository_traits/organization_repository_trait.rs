use crate::common_types::{DBTransaction, DatabaseError, OrganizationId};
use crate::models::organization_model::OrganizationModel;
use crate::models::user_model::UserId;
use crate::types::PublicId;
use async_trait::async_trait;

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
    async fn create_organization(
        &self,
        tx: &mut DBTransaction<'_>,
        new_organization: NewOrganization,
    ) -> anyhow::Result<CreatedOrganization, DatabaseError>;

    async fn get_organization_by_id(
        &self,
        tx: &mut DBTransaction<'_>,
        organization_id: OrganizationId,
    ) -> anyhow::Result<Option<OrganizationModel>, DatabaseError>;
}
