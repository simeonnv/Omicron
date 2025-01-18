use crate::{config, db::{get_db_pool::get_db_pool, queries}};


pub async fn init_tables() -> Result<(), sqlx::Error> {
    println!("init tables");

    let pool = get_db_pool();

    for query in queries::QUERIES.iter() {
        sqlx::query(query).execute(pool).await?;
    }

    println!("Database '{}' created or already exists!", config::DB_NAME);

    Ok(())

}
