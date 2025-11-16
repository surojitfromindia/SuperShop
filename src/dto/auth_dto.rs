use crate::dto::common::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use super_shop_backend::types::{PlainPassword, UserLoginError, UserLoginInput, UserLoginOutput};

#[derive(Debug, Deserialize)]
pub struct UserLoginPayload {
    email: String,
    password: String,
}

impl Into<UserLoginInput> for UserLoginPayload {
    fn into(self) -> UserLoginInput {
        UserLoginInput {
            email: self.email,
            password: PlainPassword::new(self.password),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    user_id: String,
    token: String,
}
impl From<UserLoginOutput> for UserLoginResponse {
    fn from(value: UserLoginOutput) -> Self {
        UserLoginResponse {
            user_id: value.public_id.into(),
            token: value.token.into(),
        }
    }
}

pub struct UserLoginHttpError(pub UserLoginError);
impl IntoResponse for UserLoginHttpError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self.0 {
            UserLoginError::TokenError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ),
            UserLoginError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                "invalid email".to_string(),
            ),
            UserLoginError::InvalidPassword => (
                StatusCode::BAD_REQUEST,
                "incorrect password".to_string(),
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),

        };

        let body = Json(ErrorResponse {
            error: self.0.to_string(),
            message: msg,
        });

        (status, body).into_response()
    }
}
