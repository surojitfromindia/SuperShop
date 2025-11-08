use axum::routing::get;
use axum::Router;

pub fn user_routes() -> Router {
    let router = Router::new().route("/", get(health_check));
    router
}


 async fn health_check() -> String {
    "Item routes are healthy".to_string()
}