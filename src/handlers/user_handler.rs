use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use super_shop_backend::types::RegisterUser;
use super_shop_backend::{AppState, UserService};

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    phone: Option<String>,
}
impl Into<RegisterUser> for RegisterUserPayload {
    fn into(self) -> RegisterUser {
        RegisterUser {
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
        }
    }
}

pub async fn register_user_handler(State(app_state): State<AppState>, Json(payload): Json<RegisterUserPayload>) -> String {
    let user_service = UserService { app_state };
    user_service.register_user(payload.into()).await;
    "ki".to_string()
}
