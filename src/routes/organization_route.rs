use crate::handlers::organization_handler::create_organization_handler;
use axum::Router;
use axum::routing::{get, post};
use super_shop_backend::AppState;

pub fn organization_routes() -> Router<AppState> {
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/", post(create_organization_handler));
    router
}

async fn health_check() -> String {
    "Organization routes are healthy".to_string()
}
