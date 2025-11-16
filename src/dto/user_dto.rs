use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use super_shop_backend::errors::RegisterUserError;
use super_shop_backend::types::{PlainPassword, RegisterUserInput, RegisterUserOutput};
use crate::dto::common::ErrorResponse;

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

impl From<RegisterUserPayload> for RegisterUserInput {
    fn from(val: RegisterUserPayload) -> Self {
        RegisterUserInput {
            email: val.email,
            password: PlainPassword::new(val.password),
            first_name: val.first_name,
            last_name: val.last_name,
            phone: val.phone,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub public_id: String,
    pub email: String,
}

impl From<RegisterUserOutput> for RegisterUserResponse {
    fn from(val: RegisterUserOutput) -> Self {
        RegisterUserResponse {
            email: val.email,
            public_id: val.public_id.to_string(),
        }
    }
}


pub struct RegisterUserHttpError(pub RegisterUserError);

impl IntoResponse for RegisterUserHttpError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self.0 {
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