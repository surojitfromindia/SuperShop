use crate::common_types::{
    DBTransaction, DatabaseError, OrganizationId, PublicId, PublicIdCounter, ShopDB,
};
use crate::repository_traits::branch_repository_trait::{
    BranchRepositoryTrait, CreatedBranch, NewBranch,
};
use crate::repository_traits::organization_repository_trait::{
    CreatedOrganization, NewOrganization, OrganizationRepositoryTrait,
};
use crate::types::PrimaryId;
use async_trait::async_trait;
use sqlx::Row;

pub struct BranchRepository {
    pub(crate) shop_db: ShopDB,
}

impl BranchRepository {
    pub fn new(shop_db: ShopDB) -> Self {
        Self { shop_db }
    }
}

#[async_trait]
impl BranchRepositoryTrait for BranchRepository {
    async fn create_branch_public_id_counter(
        &self,
        tx: &mut DBTransaction,
        counter: PublicIdCounter<'_>,
    ) -> anyhow::Result<(), DatabaseError> {
        sqlx::query("insert into _counter_public_id_branch (organization_id, last_value, organization_public_id) values ($1,$2,$3)")
            .bind(counter.organization_id)
            .bind(0)
            .bind(counter.organization_public_id)
            .fetch_one(tx.as_mut()).await?;
        Ok(())
    }

    async fn create_branch(
        &self,
        tx: &mut DBTransaction<'_>,
        new_branch: NewBranch,
    ) -> anyhow::Result<CreatedBranch, DatabaseError> {
        let created_branch = sqlx::query("insert into organization_branches (name, name_sl, organization_id, is_default, created_by_user_id) values ($1, $2, $3, $4) returning  id")
            .bind(new_branch.name)
            .bind(new_branch.name_sl)
            .bind(new_branch.organization_id)
            .bind(new_branch.is_default)
            .bind(new_branch.created_by_user_id)
            .fetch_one(tx.as_mut()).await?;

        let id: PrimaryId = created_branch.get("id");

        Ok(CreatedBranch { id })
    }
}
