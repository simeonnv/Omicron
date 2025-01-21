use serde::{Deserialize, Serialize};

use crate::libs::crypto;
use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;
use crate::structs::accounts;


#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}


pub async fn check_credentials(username: &String, password: &String) -> Result<(bool, i64, String), Error> {

    let pool = get_db_pool();

    let db_res = sqlx::query_as::<_, accounts::Accounts>(r#"
        SELECT * FROM Accounts 
            WHERE username = ?
        ;
    "#)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    let account = match db_res {
        Some(value) => value,
        None => return Ok((false, 0, "user".to_string())),
    };

    crypto::compare::compare(password, &account.password).await?;
    
    Ok((crypto::compare::compare(password, &account.password).await?, account.account_id, account.role))
}