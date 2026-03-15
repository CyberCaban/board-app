use rocket_sync_db_pools::database;

pub mod user_queries;
pub mod file_queries;
pub mod friend_queries;
pub mod routes_queries;
pub mod board_queries;
pub mod users_interaction_queries;

#[database("pgsql")]
pub struct Db(diesel::PgConnection);