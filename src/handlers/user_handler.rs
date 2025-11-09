use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use super_shop_backend::errors::RegisterUserError;
use super_shop_backend::types::{RegisterUserInput, RegisterUserOutput};
use super_shop_backend::{AppState, UserService};

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    phone: Option<String>,
}

impl Into<RegisterUserInput> for RegisterUserPayload {
    fn into(self) -> RegisterUserInput {
        RegisterUserInput {
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub public_id: String,
    pub email: String,
}
impl Into<RegisterUserResponse> for RegisterUserOutput {
    fn into(self) -> RegisterUserResponse {
        RegisterUserResponse {
            email: self.email,
            public_id: self.public_id.to_string(),
        }
    }
}

use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

pub struct RegisterUserHttpError(pub RegisterUserError);

impl IntoResponse for RegisterUserHttpError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self.0 {
            RegisterUserError::DatabaseError(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
            RegisterUserError::DuplicateEmail(m) => (StatusCode::CONFLICT, m.clone()),
            RegisterUserError::ValidationError(m) => (StatusCode::BAD_REQUEST, m.clone()),
            RegisterUserError::Unexpected(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: self.0.to_string(),
            message: msg,
        });

        (status, body).into_response()
    }
}

pub async fn register_user_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<Json<RegisterUserResponse>, RegisterUserHttpError> {
    let user_service = UserService { app_state };
    let user = user_service
        .register_user(payload.into())
        .await
        .map_err(RegisterUserHttpError)?;

    Ok(Json(user.into()))
}
