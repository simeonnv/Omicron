use chrono::NaiveDateTime;
use sqlx::types::chrono;

#[derive(sqlx::FromRow, Debug)]
pub struct Accounts {
    pub account_id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime
}