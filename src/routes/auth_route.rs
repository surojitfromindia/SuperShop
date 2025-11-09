use crate::handlers::auth_handler::user_login_handler;
use axum::routing::{get, post};
use axum::Router;
use super_shop_backend::AppState;

pub fn auth_routes() -> Router<AppState> {
    let router = Router::new()
        .route("/",post(user_login_handler))
        .route("/", get(health_check));
    router
}

async fn health_check() -> String {
    "Auth routes are healthy".to_string()
}
