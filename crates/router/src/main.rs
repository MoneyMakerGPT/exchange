use actix_web::{web, App, Error, HttpResponse, HttpServer};
use deadpool_postgres::Client;
use dotenvy::dotenv;

use db_postgres::{get_db_pool, models::User, db, errors::DbError};
use confik::{Configuration as _, EnvSource};

use crate::config::RouterConfig;

pub mod config;

pub async fn get_users() -> Result<HttpResponse, Error> {
    let client: Client = get_db_pool().get().await.map_err(DbError::PoolError)?;

    let users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(user: web::Json<User>) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = get_db_pool().get().await.map_err(DbError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let config = RouterConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let server = HttpServer::new(move || {
        App::new()
                .service(web::resource("/users")
                .route(web::post().to(add_user))
                .route(web::get().to(get_users)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}