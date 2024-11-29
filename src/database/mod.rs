use rocket_sync_db_pools::database;

#[database("pgsql")]
pub struct Db(diesel::PgConnection);