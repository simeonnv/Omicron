use crate::{error::Error, libs::db::get_db_pool::get_db_pool};
use sqlx;

#[derive(sqlx::FromRow, Debug)]
struct Subicron {
    count: i8,
}

pub async fn insure_subicron_exists(subicron_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    let subicron_exists: Subicron = sqlx::query_as(r#"
        SELECT COUNT(*) AS count FROM Subicrons
            WHERE subicron_id = ?
        ;
        "#)
        .bind(subicron_id)
        .fetch_one(pool)
        .await?;

    if subicron_exists.count == 1 {
        return Ok(());
    } else {
        return Err(Error::NotFound(
            "Subicron doesnt exist!".to_string()
        ))
    }

}