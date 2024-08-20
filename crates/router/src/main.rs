use actix_web::{web, App, Error, HttpResponse, HttpServer};
use deadpool_postgres::Client;

use db_postgres::{get_db_pool, models::User, db, errors::DbError};

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
    let server = HttpServer::new(move || {
        App::new()
                .service(web::resource("/users")
                .route(web::post().to(add_user))
                .route(web::get().to(get_users)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    println!("Server running at http://127.0.0.1:8080/");

    server.await
}