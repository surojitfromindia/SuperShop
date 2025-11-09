use axum::Json;
use axum::extract::State;
use super_shop_backend::{AppState, UserService};
use crate::dto::user_dto::{RegisterUserHttpError, RegisterUserPayload, RegisterUserResponse};

pub async fn register_user_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<Json<RegisterUserResponse>, RegisterUserHttpError> {
    let user_service = UserService { app_state };
    let user = user_service
        .register_user(From::from(payload))
        .await
        .map_err(RegisterUserHttpError)?;

    Ok(Json(user.into()))
}
