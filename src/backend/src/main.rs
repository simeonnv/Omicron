use libs::auth::create_account::create_account;
use libs::db;
use tokio::sync::OnceCell;

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use sqlx::{MySql, Pool};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api_docs;
pub mod config;
pub mod routes;
pub mod libs;
pub mod structs;
pub mod error;

static DB: OnceCell<Pool<MySql>> = OnceCell::const_new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    db::init_pool::init_pool().await.expect("Failed to initialize database");
    db::init_tables::init_tables().await.expect("Failed to initialize tables");
    let _ = create_account(&"admin".to_string(), &"admin".to_string(), "admin", true).await;
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))

            .service(routes::debug::health::health)
            .service(routes::debug::auth_me())
            
            .service(routes::auth::auth())
            .service(routes::subicron::subicron())
            
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()))
    })
    .bind((config::LISTENING_ON, config::PORT))?
    .run()
    .await
}