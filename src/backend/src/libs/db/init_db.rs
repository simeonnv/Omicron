use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use crate::config;


pub async fn init_db() -> Result<Pool<MySql>, sqlx::Error> {
    println!("Connecting to db");

    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(format!("mysql://{}:{}@{}", config::DB_USERNAME, config::DB_PASSWORD, config::DB_ADDRESS).as_str())
        .await?;

    println!("Connected to mysql://{}:{}@{}", config::DB_USERNAME, config::DB_PASSWORD, config::DB_ADDRESS);

    sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {}", config::DB_NAME))
        .execute(&pool)
        .await?;

    println!("Database '{}' created or already exists!", config::DB_NAME);

    let pool_with_db: Pool<MySql> = MySqlPoolOptions::new()
        .connect(
            format!("mysql://{}:{}@{}/{}", 
                config::DB_USERNAME, 
                config::DB_PASSWORD, 
                config::DB_ADDRESS,
                config::DB_NAME
            ).as_str())
        .await?;

    println!("Connected to db {}", config::DB_NAME);

    Ok(pool_with_db)
}


