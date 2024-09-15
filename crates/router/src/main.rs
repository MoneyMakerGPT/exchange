use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use dotenvy::dotenv;

use crate::config::RouterConfig;
pub mod config;

pub async fn get_route() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("Hello world!"))
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

    // println!("env var: {}", std::env::var("DATABASE_URL").unwrap());

    let server = HttpServer::new(move || {
        App::new().service(web::resource("/users").route(web::get().to(get_route)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
