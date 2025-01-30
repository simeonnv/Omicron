use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

//  this function does NOT check if the post exists
pub async fn upvote_post(post_id: i64, account_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    let _ = sqlx::query(r#"

        INSERT INTO Post_Upvotes (account_id, post_id)
        VALUES (?, ?);

    "#)
        .bind(account_id)
        .bind(post_id)
        .execute(pool)
        .await?;

    Ok(())
}