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
    pub created_at: NaiveDateTime,
    pub poster_username: String,
}

pub async fn get_post_from_id(post_id: i64, subicron_id: i64) -> Result<PostSearchRes, Error> {

    let pool = get_db_pool();

    let posts_res: Option<PostSearchRes> = sqlx::query_as(r#"

        SELECT 
            Posts.*,
            Accounts.username AS poster_username
        FROM
            Posts 
            INNER JOIN Accounts ON Posts.poster_id = Accounts.account_id
        WHERE 
            Posts.subicron_id = ? AND
            Posts.post_id = ?
        ORDER BY created_at DESC
        LIMIT 10;


    "#)
        .bind(subicron_id)
        .bind(post_id)
        .fetch_optional(pool)
        .await?;
    
    let posts = match posts_res {
        Some(e) => e,
        None => return Err(Error::NotFound("post not found".to_owned()))
    };

    Ok(posts)
}