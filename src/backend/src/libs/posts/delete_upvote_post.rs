use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

//  this function does NOT check if the post exists
pub async fn delete_upvote_post(post_id: i64, account_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    dbg!(post_id, account_id);

    let _ = sqlx::query(r#"

        DELETE FROM Post_Upvotes
        WHERE account_id = ? AND post_id = ?;

    "#)
        .bind(account_id)
        .bind(post_id)
        .execute(pool)
        .await?;

    Ok(())
}