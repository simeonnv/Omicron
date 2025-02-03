use utoipa::ToSchema;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug, serde::Serialize, ToSchema)]
pub struct GetUpvotesRes {
    pub upvotes: i64,
    pub is_upvoted: bool
}

pub async fn get_upvotes(account_id: i64, post_id: i64) -> Result<GetUpvotesRes, Error> {

    let pool = get_db_pool();

    let upvote_res: Option<GetUpvotesRes> = sqlx::query_as(r#"

        SELECT
            COUNT(*) AS upvotes,
            COUNT(CASE WHEN account_id = ? THEN 1 END) AS is_upvoted
        FROM
            Post_Upvotes
        WHERE
            post_id = ?
        HAVING COUNT(*) > 0;
            
    "#)
        .bind(&account_id)
        .bind(&post_id)
        .fetch_optional(pool)
        .await?;

    let upvote = match upvote_res {
        Some(e) => e,
        None => return Err(Error::NotFound("there is no such post/subicron".to_owned()))
    };
    
    Ok(upvote)
}