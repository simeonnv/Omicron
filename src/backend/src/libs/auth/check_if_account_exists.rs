use serde::{Deserialize, Serialize};

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;


#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str
}


pub async fn check_if_account_exists(username: &String) -> Result<bool, Error> {

    let pool = get_db_pool();

    #[derive(sqlx::FromRow, Debug)]
    struct User { count: i8 }

    let account = sqlx::query_as::<_, User>(r#"
        SELECT COUNT(*) AS count 
            FROM Accounts 
            WHERE username = ?
        ;
    "#)
        .bind(username)
        .fetch_one(pool)
        .await?;
    
    Ok(account.count > 0)
}