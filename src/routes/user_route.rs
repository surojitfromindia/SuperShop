use crate::handlers::user_handler::register_user_handler;
use axum::Router;
use axum::routing::{get, post};
use super_shop_backend::AppState;

pub fn user_routes() -> Router<AppState> {
    let router = Router::new()
        .route("/",post(register_user_handler))
        .route("/", get(health_check));
    router
}

async fn health_check() -> String {
    "User routes are healthy".to_string()
}
