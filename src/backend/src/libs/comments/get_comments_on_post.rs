use chrono::NaiveDateTime;
use serde::Serialize;
use crate::{error::Error, libs::db::get_db_pool::get_db_pool};

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Comment {
    pub comment_id: i64,
    pub text: String,
    pub embed_id: Option<i64>,
    pub commenter_id: i64,
    pub created_at: NaiveDateTime,
    pub commenter_username: String
}

pub async fn get_comments_on_post(post_id: i64, page: i64) -> Result<Vec<Comment>, Error> {

    let pool = get_db_pool();

    let posts_res: Vec<Comment> = sqlx::query_as(r#"

        SELECT 
            Comments.comment_id, Comments.`text`,
            Comments.embed_id, Comments.commenter_id, Comments.created_at,
            Accounts.username AS commenter_username
        FROM 
            Comments
            INNER JOIN Accounts ON Comments.commenter_id = Accounts.account_id
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