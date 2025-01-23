use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct SubicronCreationRes {
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn create_subicron(name: &String, image_id: Option<i64>) -> Result<(), Error> {

    let pool = get_db_pool();

    sqlx::query(r#"
        
        INSERT INTO Subicrons
            (image_id, name)
            VALUES(?, ?)
        RETURNING created_at;
        

    "#)
        .bind(image_id)
        .bind(name)
        .execute(pool)
        .await?;
    
    Ok(())
}