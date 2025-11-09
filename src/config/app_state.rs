use crate::config::db_config::{connect_to_db, DbConfig};
use crate::config::load_env::{load_env, EnvConfig};
use crate::repositories::user_repository::UserRepository;
use std::process::exit;
use std::sync::Arc;

pub struct Repositories {
    pub user_repository: UserRepository,
}

#[derive(Clone)]
pub struct AppState {
    // pub shop_db: ShopDB,
    pub env_config: EnvConfig,
    pub repositories: Arc<Repositories>,
}

impl AppState {
    pub async fn init(db_config: DbConfig<'_>) -> Self {
        // connect to shop db or on failer shutdown
        let shop_db = connect_to_db(db_config).await.unwrap_or_else(|_| {
            eprintln!("Failed to connect to db!");
            exit(1);
        });
        // try to load env data.
        let env_config = load_env().unwrap_or_else(|e| {
            eprintln!("Failed to load env config! {}", e);
            exit(1);
        });

        let user_repository = UserRepository::new(shop_db.clone());

        // later store user repository.
        AppState {
            // shop_db,
            env_config,
            repositories: Arc::new(Repositories { user_repository }),
        }
    }
}
