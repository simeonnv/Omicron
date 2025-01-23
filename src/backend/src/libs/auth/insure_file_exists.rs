use crate::{error::Error, libs::db::get_db_pool::get_db_pool};
use sqlx;

#[derive(sqlx::FromRow, Debug)]
struct File {
    count: i8,
}

pub async fn insure_file_exists(file_id: i64) -> Result<(), Error> {

    let pool = get_db_pool();

    let subicron_exists: File = sqlx::query_as(r#"
        SELECT COUNT(*) AS count FROM Files
            WHERE file_id = ?
        ;
        "#)
        .bind(file_id)
        .fetch_one(pool)
        .await?;

    if subicron_exists.count == 1 {
        return Ok(());
    } else {
        return Err(Error::BadRequest(
            "File/Embed doesnt exist!".to_string()
        ))
    }

}