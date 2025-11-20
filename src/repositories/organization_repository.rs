use crate::common_types::{DBTransaction, DatabaseError, OrganizationId, PublicId, ShopDB};
use crate::repository_traits::organization_repository_trait::{
    CreatedOrganization, NewOrganization, OrganizationRepositoryTrait,
};
use async_trait::async_trait;
use sqlx::Row;
use crate::models::organization_model::OrganizationModel;

pub struct OrganizationRepository {
    pub(crate) shop_db: ShopDB,
}

impl OrganizationRepository {
    pub fn new(shop_db: ShopDB) -> Self {
        Self { shop_db }
    }
}

#[async_trait]
impl OrganizationRepositoryTrait for OrganizationRepository {
    async fn create_organization(
        &self,
        tx: &mut DBTransaction<'_>,
        new_organization: NewOrganization,
    ) -> anyhow::Result<CreatedOrganization, DatabaseError> {
        // create organization.
        let created_organization = sqlx::query(
            "insert into organizations (name, name_sl, created_by_user_id) values ($1, $2, $3) returning id, public_id"
        ).bind(new_organization.name)
            .bind(new_organization.name_sl)
            .bind(new_organization.created_by_user_id)
            .fetch_one(tx.as_mut()).await?;
        let id: OrganizationId = created_organization.get("id");
        let public_id: PublicId = created_organization.get("public_id");

        Ok(CreatedOrganization { public_id, id })
    }

    async fn get_organization_by_id(&self, tx: &mut DBTransaction<'_>, organization_id: OrganizationId) -> anyhow::Result<Option<OrganizationModel>, DatabaseError> {
        let organization: Option<OrganizationModel> = sqlx::query_as("select * from organizations where id = $1")
            .bind(organization_id)
            .fetch_optional(tx.as_mut()).await?;
        Ok(organization)
    }
}
