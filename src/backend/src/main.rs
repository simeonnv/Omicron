use once_cell::sync::Lazy;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use sqlx::{MySql, Pool};

pub mod config;
pub mod db;
pub mod routes;

static DB: Lazy<Pool<MySql>> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(db::init_db::init_db())
        .expect("Failed to initialize the database")
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))

            .service(routes::health::health)
            
    })
    .bind((config::LISTENING_ON, config::PORT))?
    .run()
    .await
}