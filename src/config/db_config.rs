use std::str::FromStr;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool, Pool};

pub struct DbConfig<'a> {
    pub db_url : &'a str
}

pub type ShopDB = PgPool;

pub async fn connect_to_db(db_config: DbConfig<'_>)-> anyhow::Result<ShopDB> {
    let connection_options = PgConnectOptions::from_str(db_config.db_url)
        .expect("Failed to create connection options!")
        .log_statements(log::LevelFilter::Trace);
    let db = PgPool::connect_with(connection_options).await?;
    println!("db connection established!");
    Ok(db)
}