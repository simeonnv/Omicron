use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct SubicronCreationRes {
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn post_file(file_blob: &Vec<u8>, account_id: i64) -> Result<i64, Error> {

    let pool = get_db_pool();

    let file_type = infer::get(file_blob)
        .map(|t| t.mime_type()) // Get MIME type (e.g., "image/png")
        .unwrap_or("unknown");

    let file_size = file_blob.len() as i64;

    let file_id: (i64,) = sqlx::query_as(r#"
        
        INSERT INTO Files
        (file_blob, `size`, file_type, account_id)
        VALUES(?, ?, ?, ?)
        RETURNING file_id;
        
    "#)
        .bind(file_blob)
        .bind(file_size)
        .bind(file_type)
        .bind(account_id)
        .fetch_one(pool)
        .await?;
    
    Ok(file_id.0)
}