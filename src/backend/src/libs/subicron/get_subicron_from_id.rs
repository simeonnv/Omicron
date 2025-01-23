use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

use super::search_for_subicron::SubicronSearchRes;

pub async fn get_subicron_from_id(id: i64) -> Result<Option<SubicronSearchRes>, Error> {

    let pool = get_db_pool();

    let subicrons: Option<SubicronSearchRes> = sqlx::query_as(r#"

        SELECT image_id, name, created_at, subicron_id
        FROM Subicrons
        WHERE subicron_id = ?;

    "#)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    
    Ok(subicrons)
}