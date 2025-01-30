use chrono::NaiveDateTime;
use serde::Serialize;
use crate::{error::Error, libs::db::get_db_pool::get_db_pool};

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Comment {
    pub text: String,
    pub embed_id: Option<i64>,
    pub commenter_id: i64,
    pub created_at: NaiveDateTime
}



pub async fn create_comment_on_post(post_id: i64, account_id: i64, text: String, embed_id: Option<i64>) -> Result<(), Error> {

    let pool = get_db_pool();

    sqlx::query(r#"

        INSERT INTO Omicron.Comments
            (`text`, embed_id, commenter_id, post_id)
        VALUES(?, ?, ?, ?);

    "#)
        .bind(text)
        .bind(embed_id)
        .bind(account_id)
        .bind(post_id)
        .execute(pool)
        .await?;

    Ok(())
}