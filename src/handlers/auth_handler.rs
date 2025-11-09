use crate::dto::auth_dto::{UserLoginHttpError, UserLoginPayload, UserLoginResponse};
use axum::extract::State;
use axum::Json;
use super_shop_backend::errors::AuthService;
use super_shop_backend::AppState;

pub async fn user_login_handler(
    State(app_state): State<AppState>,
    Json(payload) : Json<UserLoginPayload>
)->anyhow::Result<Json<UserLoginResponse>,UserLoginHttpError> {
    let auth_service = AuthService {
        app_state,
    };
    let login = auth_service
        .user_login(payload.into())
        .await
        .map_err(UserLoginHttpError)?;
    Ok(Json(login.into()))
}
