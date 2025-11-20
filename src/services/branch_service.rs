use crate::common_types::{DBTransaction, PublicIdCounter};
use crate::models::organization_model::OrganizationModel;
use crate::repositories::branch_repository::BranchRepository;
use crate::repository_traits::branch_repository_trait::{BranchRepositoryTrait, NewBranch};
use crate::{AccessContext, AppState};

pub struct BranchService {
    pub app_state: AppState,
    pub access_context: AccessContext,
}

impl BranchService {
    pub async fn init_default(
        &self,
        tx: &mut DBTransaction<'_>,
        organization: &OrganizationModel,
    ) -> anyhow::Result<()> {
        let app_state = &self.app_state;

        let branch_repository = BranchRepository {
            shop_db: app_state.shop_db.clone(),
        };

        // 1. create the branch public counter.
        branch_repository
            .create_branch_public_id_counter(
                tx,
                PublicIdCounter {
                    organization_id: &organization.id,
                    organization_public_id: &organization.public_id,
                },
            )
            .await?;

        // 2. create a default branch
        branch_repository
            .create_branch(
                tx,
                NewBranch {
                    name: "Head office".to_string(),
                    name_sl: Some("Head office".to_string()),
                    organization_id: 0,
                    created_by_user_id: self.access_context.get_current_accessor_id().clone(),
                    is_default: Some(true),
                },
            )
            .await?;

        Ok(())
    }
}
