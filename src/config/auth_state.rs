use crate::common_types::{OrganizationId, UserId};
use crate::types::PublicId;
use crate::utils::token::{Token, TokenVerificationError};
use crate::{AppState, UserService};
use crate::errors::AuthService;
use crate::services::auth_service::UserAccessContext;

enum AppAccessMode {
    APP,
    USER,
    CHRON,
}

struct AuthContextInput {
    token: String,
    app_state: AppState,
}
struct AuthContext {
    access_mode: AppAccessMode,
    public_user_id: Option<PublicId>,
    app_id: Option<String>,
}


enum AuthContextInputError {
    AuthenticationFailed(String),
    TokenError(TokenVerificationError),
}

// AuthContext need to be created by the handles and to be passed in service.
impl AuthContext {
    pub async fn init(inp: AuthContextInput) -> Result<Self, AuthContextInputError> {
        // note: no invalid user state error should be thrown after this function exit.

        // verify token.
        let claims =  Token::verify(&inp.token, &inp.app_state.env_config)
            .map_err(|e| AuthContextInputError::TokenError(e))?;

        let pubic_user_id_from_token : PublicId = claims.user_id.try_into().expect("invalid public user id");

        Ok(AuthContext {
            access_mode: AppAccessMode::USER,
            public_user_id: Some(pubic_user_id_from_token),
            app_id: None,
        })
    }
}

// todo: I need another struct that actually use this AuthContext to
// get actual data. like org_id, permissions etc.

struct AccessContext{
    pub auth_context: AuthContext,
    organization_id: OrganizationId,
    user_access_context : Option<UserAccessContext>,
}
impl AccessContext {
    async fn init(auth_context: AuthContext, app_state: AppState)-> anyhow::Result<()> {
        let AuthContext { public_user_id, .. } = auth_context;

        let user= match public_user_id {
            None => None,
            Some(public_user_id) => {
                let auth_service = AuthService{
                    app_state,
                };
                auth_service.get_user_for_access_context(public_user_id).await?
            }
        };

        Ok(())
    }


    fn is_user(&self) -> bool {
        self.user_access_context.is_some()
    }
}