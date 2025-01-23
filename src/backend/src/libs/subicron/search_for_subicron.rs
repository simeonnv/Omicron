use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct SubicronSearchRes {
    pub image_id: Option<i64>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn search_for_subicron(name: &String) -> Result<Vec<SubicronSearchRes>, Error> {

    let pool = get_db_pool();

    let search_query = format!("%{}%", name);

    let subicrons: Vec<SubicronSearchRes> = sqlx::query_as(r#"

        SELECT image_id, name, created_at, subicron_id
        FROM Subicrons
        WHERE name LIKE ?
        ORDER BY created_at DESC
        LIMIT 10;

    "#)
        .bind(search_query)
        .fetch_all(pool)
        .await?;
    
    Ok(subicrons)
}