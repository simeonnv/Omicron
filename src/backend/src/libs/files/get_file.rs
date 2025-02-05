use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;
use crate::structs::files::Files;

#[derive(sqlx::FromRow, Debug)]
pub struct SubicronCreationRes {
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn get_file(file_id: i64) -> Result<Files, Error> {

    let pool = get_db_pool();

    
    let file: Files = sqlx::query_as(r#"
        
        SELECT file_id, file_blob, `size`, file_type, account_id, created_at
        FROM Files
        WHERE file_id = ?;
        
    "#)
        .bind(file_id)
        .fetch_one(pool)
        .await?;
    
    Ok(file)
}