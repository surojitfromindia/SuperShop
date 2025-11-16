use crate::dto::auth_dto::UserLoginPayload;
use crate::handlers::auth_extractor::AccessContextExtractor;
use axum::extract::State;
use axum::Json;
use super_shop_backend::AppState;

pub async fn create_organization_handler(
    State(app_state): State<AppState>,
    AccessContextExtractor(access_ctx): AccessContextExtractor,
) -> String {
    println!("access_ctx is {:?}", access_ctx);
    "Organization is created".to_string()
}
