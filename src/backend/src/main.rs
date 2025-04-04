use std::{fs::File, io::BufReader};

use actix_cors::Cors;
use libs::auth::create_account::create_account;
use libs::db;
use tokio::sync::OnceCell;

use actix_web::{middleware::Logger, web, App, HttpServer};
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

    // rustls::crypto::aws_lc_rs::default_provider()
    //     .install_default()
    //     .unwrap();

    // let mut certs_file = BufReader::new(File::open(config::CERT_PATH).unwrap());
    // let mut key_file = BufReader::new(File::open(config::KEY_PATH).unwrap());

    // let tls_certs = rustls_pemfile::certs(&mut certs_file)
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();
    // let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
    //     .next()
    //     .unwrap()
    //     .unwrap();

    // let tls_config = rustls::ServerConfig::builder()
    //     .with_no_client_auth()
    //     .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
    //     .unwrap();

    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_origin() // Allow any origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]) // Allow all methods
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
        
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))

            
            // .service(routes::debug::health::health)
            // .service(routes::debug::auth_me())
            
            .service(routes::auth::auth())
            .service(routes::subicron::subicron())

            .service(routes::files::files())
            
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()))
    })
    .bind((config::LISTENING_ON, config::PORT))?
    // .bind_rustls_0_23((config::LISTENING_ON, config::PORT), tls_config)?
    .run()
    .await
}