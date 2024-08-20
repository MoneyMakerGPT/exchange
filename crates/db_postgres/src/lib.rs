use confik::{Configuration as _, EnvSource};
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::config::PostgresConfig;

pub mod config;
pub mod db;
pub mod errors;
pub mod models;

pub fn get_db_pool() -> Pool {
    dotenv().ok();

    let config = PostgresConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    config.pg.create_pool(None, NoTls).unwrap()
}
