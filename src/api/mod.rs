use diesel::SqliteConnection;
use rocket_contrib::database;

mod app;
mod views;
pub use app::launch;

#[database("db")]
pub struct DBConn(SqliteConnection);
