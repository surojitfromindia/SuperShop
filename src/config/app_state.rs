use crate::common_types::ShopDB;
use crate::config::db_config::{connect_to_db, DbConfig};
use crate::config::load_env::{load_env, EnvConfig};
use std::process::exit;


#[derive(Clone)]
pub struct AppState {
    pub shop_db: ShopDB,
    pub env_config: EnvConfig,
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


        AppState {
            shop_db,
            env_config,
        }
    }
}
