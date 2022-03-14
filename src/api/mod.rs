use diesel::SqliteConnection;
use rocket_sync_db_pools::database;

pub mod views;

#[database("db")]
pub struct DBConn(SqliteConnection);
