use serde::{Deserialize, Serialize};

use crate::{libs::crypto, libs::db::get_db_pool::get_db_pool};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}

pub async fn create_account(username: &String, password: &String, role: &'static str, ignore: bool) -> Result<(String, i64), Error> {

    let pool = get_db_pool();

    let hashed_password = crypto::hash::hash(password).await?;

    let account: (String, i64);
    if ignore {
        account = sqlx::query_as(r#"
        
            INSERT IGNORE INTO Accounts 
                (role, username, password)
                VALUES (?, ?, ?)
            RETURNING password, account_id;

        "#)
            .bind(role)
            .bind(username)
            .bind(hashed_password)
            .fetch_one(pool)
            .await?;
    } else {
        account = sqlx::query_as(r#"
        
            INSERT INTO Accounts 
                (role, username, password)
                VALUES (?, ?, ?)
            RETURNING password, account_id;

        "#)
            .bind(role)
            .bind(username)
            .bind(hashed_password)
            .fetch_one(pool)
            .await?;
    }
    
    Ok(account)
}