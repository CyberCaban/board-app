use catchers::Catcher;
use config::from_env;
use database::Db;
use rocket::{
    figment::{
        providers::{Env, Format, Toml},
        Figment,
    },
    Config,
};
use routes::AuthorizationRoutes;
use std::collections::HashMap;

mod catchers;
mod config;
mod database;
mod errors;
mod models;
mod routes;
mod schema;
mod services;

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
        // .mount_board_routes()
        .launch()
        .await?;

    Ok(())
}
