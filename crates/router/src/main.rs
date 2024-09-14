use actix_web::{web, App, Error, HttpResponse, HttpServer};
use deadpool_postgres::Client;
use dotenvy::dotenv;
use confik::{Configuration as _, EnvSource};

use db_postgres::{get_db_pool, models::User, db, errors::DbError};
use redis::{enqueue_message, dequeue_message, publish_message, subscribe_to_channel};

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

pub async fn get_message() -> Result<HttpResponse, Error> {
    let message = dequeue_message().await?;

    Ok(HttpResponse::Ok().json(message))
}

pub async fn post_message() -> Result<HttpResponse, Error> {
    let message = enqueue_message().await?;

    Ok(HttpResponse::Ok().json(message))
}

pub async fn publish() -> Result<HttpResponse, Error> {
    let message = publish_message().await?;

    Ok(HttpResponse::Ok().json(message))
}

pub async fn subscribe() -> Result<HttpResponse, Error> {
    let message = subscribe_to_channel().await?;

    Ok(HttpResponse::Ok().json(message))
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
                    .route(web::get().to(get_users))
                )
                .service(web::resource("/redis")
                    .route(web::post().to(post_message))
                    .route(web::get().to(get_message))
                )
                .service(web::resource("/pubsub")
                    .route(web::post().to(publish))
                    .route(web::get().to(subscribe))
                )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}