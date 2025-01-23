use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct SubicronCreationRes {
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn create_post(header: &String, body: &String, embed: Option<i64>, poster_id: i64, subicron_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    sqlx::query(r#"
        
        INSERT INTO Posts
        (header, body, embed_id, poster, subicron)
        VALUES(?, ?, ?, ?, ?);
        
    "#)
        .bind(header)
        .bind(body)
        .bind(embed)
        .bind(poster_id)
        .bind(subicron_id)
        .execute(pool)
        .await?;
    
    Ok(())
}