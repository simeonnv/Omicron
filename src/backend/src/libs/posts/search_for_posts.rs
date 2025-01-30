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
    pub created_at: NaiveDateTime,
}

pub async fn search_for_posts(query: &String, subicron_id: i64) -> Result<Vec<PostsSearchRes>, Error> {

    let pool = get_db_pool();

    let search_query = format!("%{}%", query);

    let subicrons: Vec<PostsSearchRes> = sqlx::query_as(r#"

        SELECT 
            Posts.*, 
            COUNT(Post_Upvotes.post_id) AS upvotes
        FROM
            Posts 
            LEFT JOIN Post_Upvotes ON Posts.post_id = Post_Upvotes.post_id 
        WHERE 
            Posts.header LIKE ? OR
            Posts.body LIKE ? AND
            Posts.subicron_id = ?
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