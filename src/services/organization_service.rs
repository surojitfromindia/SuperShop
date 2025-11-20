use crate::{AccessContext, AppState};
use crate::config::db_config::start_transaction;
use crate::repositories::organization_repository::OrganizationRepository;
use crate::repository_traits::organization_repository_trait::{NewOrganization, OrganizationRepositoryTrait};

pub struct OrganizationService {
    pub app_state: AppState,
    pub access_context: AccessContext,
}


pub struct OrganizationCreateInput {
    pub name: String,
    pub name_sl: Option<String>,
}

impl OrganizationService {
    pub async fn create_organization(&self, organization_create_input: OrganizationCreateInput) -> anyhow::Result<()> {
        let app_state = &self.app_state;
        let mut tx = start_transaction(&app_state.shop_db).await?;

        let organization_repository = OrganizationRepository {
            shop_db: app_state.shop_db.clone(),
        };

        organization_repository.create_organization(&mut tx, NewOrganization {
            name: organization_create_input.name,
            name_sl: organization_create_input.name_sl,
            created_by_user_id: 0,
        }).await?;

        tx.rollback().await?;
        // todo: create a default branch
        // todo: add as member

        Ok(())
    }

    pub async fn create_default_branch() {}
}