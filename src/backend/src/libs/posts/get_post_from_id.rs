use chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct PostSearchRes {
    pub post_id: i64, 
    pub header: String, 
    pub body: String, 
    pub embed_id: Option<i64>, 
    pub poster_id: i64, 
    pub subicron_id: i64, 
    pub upvotes: i64,
    pub created_at: NaiveDateTime
}

pub async fn get_post_from_id(post_id: i64, subicron_id: i64) -> Result<Option<PostSearchRes>, Error> {

    let pool = get_db_pool();

    let subicrons: Option<PostSearchRes> = sqlx::query_as(r#"

        SELECT post_id, header, body, embed_id, poster_id, subicron_id, upvotes, created_at
        FROM Posts
        WHERE 
            subicron_id = ? AND
            post_id = ?
        ;

    "#)
        .bind(subicron_id)
        .bind(post_id)
        .fetch_optional(pool)
        .await?;
    
    Ok(subicrons)
}