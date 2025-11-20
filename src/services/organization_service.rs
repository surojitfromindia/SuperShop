use crate::common_types::{DBTransaction, DatabaseError, OrganizationId};
use crate::config::db_config::start_transaction;
use crate::models::organization_model::OrganizationModel;
use crate::repositories::organization_repository::OrganizationRepository;
use crate::repository_traits::organization_repository_trait::{
    NewOrganization, OrganizationRepositoryTrait,
};
use crate::services::branch_service::BranchService;
use crate::{AccessContext, AppState};
use std::collections::HashMap;

pub struct OrganizationService {
    pub app_state: AppState,
    pub access_context: AccessContext,
}

pub struct OrganizationCreateInput {
    pub name: String,
    pub name_sl: Option<String>,
}

enum OrganizationErrors {
    NotFound,
    DbError(DatabaseError),
    OtherModuleInitial(String),
}

impl OrganizationService {
    pub async fn create_organization(
        &self,
        organization_create_input: OrganizationCreateInput,
    ) -> anyhow::Result<(), OrganizationErrors> {
        let app_state = &self.app_state;
        let mut tx = start_transaction(&app_state.shop_db)
            .await
            .map_err(OrganizationErrors::DbError)?;

        let organization_repository = OrganizationRepository {
            shop_db: app_state.shop_db.clone(),
        };

        let created_org = organization_repository
            .create_organization(
                &mut tx,
                NewOrganization {
                    name: organization_create_input.name,
                    name_sl: organization_create_input.name_sl,
                    created_by_user_id: 0,
                },
            )
            .await
            .map_err(OrganizationErrors::DbError)?;

        let org = organization_repository
            .get_organization_by_id(&mut tx, created_org.id)
            .await
            .map_err(OrganizationErrors::DbError)?
            .ok_or(OrganizationErrors::NotFound)?;

        self.init_default_data(&mut tx, org)
            .await
            .map_err(|e| OrganizationErrors::OtherModuleInitial(e.to_string()))?;

        tx.rollback().await.map_err(OrganizationErrors::DbError)?;
        // todo: create a default branch
        // todo: add as member

        Ok(())
    }

    async fn init_default_data(
        &self,
        tx: &mut DBTransaction<'_>,
        organization: OrganizationModel,
    ) -> anyhow::Result<()> {
        // todo: init branch
        let branch_service = BranchService {
            app_state: self.app_state.clone(),
            access_context: self.access_context.clone(),
        };
        branch_service.init_default(tx, &organization).await?;

        // todo: init role
        // todo: init organization staff

        // todo: assign user to organization staff.
        Ok(())
    }
}
