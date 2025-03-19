use rocket_sync_db_pools::database;

pub mod user_queries;
pub mod file_queries;

#[database("pgsql")]
pub struct Db(diesel::PgConnection);