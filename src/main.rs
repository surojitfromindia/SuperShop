use std::env;
use axum::Router;
use super_shop_backend::{AppState, DbConfig};
use crate::routes::user_route::user_routes;

mod routes;
mod handlers;
mod dto;


#[tokio::main]
async fn main() {

    dotenvy::dotenv().expect("Failed to load .env file");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let db_config = DbConfig { db_url : db_url.as_str()};
    
    let app_state = AppState::init(db_config).await;

    let app = Router::new()
        .nest("/api/users", user_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5020").await.unwrap();

    axum::serve(listener, app).await.unwrap();

}
