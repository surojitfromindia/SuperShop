use std::process::exit;
use crate::config::db_config::{connect_to_db, DbConfig, ShopDB};
use crate::config::load_env::{load_env, EnvConfig};

#[derive(Clone)]
pub struct AppState {
    shop_db: ShopDB,
    env_config: EnvConfig
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
            eprintln!("Failed to load env config! {}",e);
            exit(1);
        });
        // later store user repository.
        AppState {
            shop_db,
            env_config
        }
    }
}