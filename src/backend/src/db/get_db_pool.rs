use sqlx::{MySql, Pool};

use crate::DB;

pub fn get_db_pool() -> &'static Pool<MySql> {
    DB.get().expect("Database pool is not initialized")
}