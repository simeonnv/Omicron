use sqlx::types::chrono::NaiveDateTime;

use crate::libs::db::get_db_pool::get_db_pool;
use crate::error::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct SubicronCreationRes {
    pub created_at: NaiveDateTime,
    pub subicron_id: i64
}

pub async fn create_subicron(name: &String, image_id: Option<i64>) -> Result<SubicronCreationRes, Error> {

    let pool = get_db_pool();

    let group: SubicronCreationRes = sqlx::query_as(r#"
        
        INSERT INTO Subicron
            (image_id, name)
            VALUES(?, ?)
        RETURNING created_at, subicron_id;
        

    "#)
        .bind(name)
        .bind(image_id)
        .fetch_one(pool)
        .await?;
    
    Ok(group)
}