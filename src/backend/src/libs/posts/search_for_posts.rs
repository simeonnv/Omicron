use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct PostsSearchRes {
    pub post_id: i64,
    pub header: String,
    pub body: String,
    pub embed_id: Option<i64>,
    pub poster_id: i64,
    pub subicron_id: i64,
    pub upvotes: i64,
    pub poster_username: String,
    pub created_at: NaiveDateTime,
    pub is_upvoted: bool
}

pub async fn search_for_posts(query: &String, subicron_id: i64) -> Result<Vec<PostsSearchRes>, Error> {

    let pool = get_db_pool();

    let search_query = format!("%{}%", query);

    let subicrons: Vec<PostsSearchRes> = sqlx::query_as(r#"

        SELECT 
            Posts.*,
            Accounts.username AS poster_username,
            (SELECT COUNT(*) FROM Post_Upvotes WHERE Post_Upvotes.post_id = Posts.post_id) AS upvotes,
            (SELECT COUNT(*) FROM Post_Upvotes WHERE Post_Upvotes.account_id = Posts.poster_id AND Post_Upvotes.post_id = Posts.post_id) AS is_upvoted
        FROM
            Posts 
            INNER JOIN Accounts ON Posts.poster_id = Accounts.account_id
        WHERE 
            (Posts.header LIKE ? OR
            Posts.body LIKE ?)
            AND Posts.subicron_id = ?
        ORDER BY upvotes DESC
        LIMIT 10;

		       


    "#)
        .bind(&search_query)
        .bind(&search_query)
        .bind(subicron_id)
        .fetch_all(pool)
        .await?;
    
    Ok(subicrons)
}