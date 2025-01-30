use crate::{error::Error, libs::db::get_db_pool::get_db_pool};
use sqlx;

#[derive(sqlx::FromRow, Debug)]
struct Post {
    count: i8,
}

pub async fn insure_post_exists(subicron_id: i64, post_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    let post_exists: Post = sqlx::query_as(r#"
        SELECT COUNT(*) AS count FROM Posts
            WHERE 
            subicron_id = ? AND
            post_id = ?
        ;
        "#)
        .bind(subicron_id)
        .bind(post_id)
        .fetch_one(pool)
        .await?;

    if post_exists.count == 1 {
        return Ok(());
    } else {
        return Err(Error::NotFound(
            "Post doesnt exist!".to_string()
        ))
    }

}