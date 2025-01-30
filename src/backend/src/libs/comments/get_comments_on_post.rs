use chrono::NaiveDateTime;
use serde::Serialize;
use crate::{error::Error, libs::db::get_db_pool::get_db_pool};

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Comment {
    pub comment_id: i64,
    pub text: String,
    pub embed_id: Option<i64>,
    pub commenter_id: i64,
    pub created_at: NaiveDateTime
}

pub async fn get_comments_on_post(post_id: i64, page: i64) -> Result<Vec<Comment>, Error> {

    let pool = get_db_pool();

    let posts_res: Vec<Comment> = sqlx::query_as(r#"

        SELECT comment_id, `text`, embed_id, commenter_id, created_at
        FROM Comments
        WHERE post_id = ?
        ORDER BY created_at DESC
        LIMIT ?;

    "#)
        .bind(post_id)
        .bind(page * 50)
        .fetch_all(pool)
        .await?;

    Ok(posts_res)
}