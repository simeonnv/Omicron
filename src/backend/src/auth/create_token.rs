
use serde::{Deserialize, Serialize};

use crate::{crypto::{self}, db::get_db_pool::get_db_pool, error::Error};

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}

pub async fn create_token(account_id: &i64, role: &'static str) -> Result<String, Error> {

    let pool = get_db_pool();

    let token = crypto::rand_string::rand_string(64);

    dbg!(&token);

    let hashed_token: String = match crypto::hash::hash(&token).await {
        Ok(res) => res,
        Err(e) => {
            return Err(Error(format!("hash error: {}", e)))
        }
    };

    let db_res = sqlx::query(r#"
        
        INSERT INTO Tokens 
            (role, token, user_id)
            VALUES (?, ?, ?)
        RETURNING created_at;

    "#)
        .bind(role)
        .bind(hashed_token)
        .bind(account_id)
        .fetch_one(pool)
        .await;

    match db_res {
        Ok(_) => { return Ok(token) },
        Err(e) => {
            return Err(Error(format!("database: {}", e)))
        }
    };
}