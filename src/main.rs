use catchers::Catcher;
use config::from_env;
use database::Db;
use routes::AuthorizationRoutes;

mod catchers;
mod config;
mod database;
mod errors;
mod models;
mod routes;
mod schema;
mod jwt;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();

    let _rocket = rocket::custom(from_env())
        .attach(Db::fairing())
        .mount_uploads()
        .mount_catchers()
        .mount_auth_routes()
        .mount_board_routes()
        .manage_state()
        .mount_metrics()
        .launch()
        .await?;

    Ok(())
}
