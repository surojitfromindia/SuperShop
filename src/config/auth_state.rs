use crate::common_types::{OrganizationId, UserId};
use crate::errors::AuthService;
use crate::services::auth_service::UserAccessContext;
use crate::types::PublicId;
use crate::utils::token::{Token, TokenVerificationError};
use crate::{AppState, UserService};
use std::sync::Arc;
#[derive(Debug)]

enum AppAccessMode {
    APP,
    USER,
    CHRON,
}

pub struct AuthContextInput {
    pub token: String,
    pub app_state: AppState,
}

#[derive(Debug)]
struct AuthContext {
    token: String,
    access_mode: AppAccessMode,
    public_user_id: Option<PublicId>,
    app_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AccessContext {
    auth_context: Arc<AuthContext>,
    // this context will only available if caller of this api is a human
    user_access_context: Arc<Option<UserAccessContext>>,
}

impl AccessContext {
    pub async fn init(inp: AuthContextInput, app_state: AppState) -> anyhow::Result<AccessContext> {
        let AuthContextInput { token, .. } = inp;

        // verify token.
        let claims = Token::verify(&token, &inp.app_state.env_config)?;
        // todo: later we may not have user_id in claims
        let public_user_id: Option<PublicId> = Some(claims.user_id.into());

        let user_access_context = match public_user_id.as_ref() {
            None => None,
            Some(public_user_id) => {
                let auth_service = AuthService { app_state };
                auth_service
                    .get_user_for_access_context(public_user_id)
                    .await?
            }
        };

        Ok(AccessContext {
            auth_context: Arc::new(AuthContext {
                token,
                access_mode: AppAccessMode::USER,
                public_user_id,
                app_id: None,
            }),
            user_access_context: Arc::new(user_access_context),
        })
    }

    fn is_user(&self) -> bool {
        self.user_access_context.is_some()
    }

    pub fn get_current_accessor_id(&self) -> &UserId {
        let n = &self
            .user_access_context
            .as_ref() // move into the arc
            .as_ref() // get ref from the option.
            .expect("No user access context")
            .id;
        n
    }
}
