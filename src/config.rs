use dotenv::dotenv;
use rocket::{
    data::{Limits, ToByteUnit},
    figment::{
        providers::Env,
        value::magic::RelativePathBuf,
        Figment,
    },
    Config,
};
use std::{collections::HashMap, env};

pub fn from_env() -> Figment {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:123456@localhost:5432/".to_string());

    Config::figment()
        .merge(("port", port.parse::<u16>().unwrap()))
        .merge(("address", address))
        .merge((
            "limits",
            Limits::default()
                .limit("file", 2.gigabytes())
                .limit("data-form", 2.gibibytes()),
        ))
        .merge(("temp_dir", RelativePathBuf::from("tmp")))
        .merge(Env::raw())
        .merge((
            "databases",
            HashMap::from([("pgsql", HashMap::from([("url", db_url)]))]),
        ))
}
