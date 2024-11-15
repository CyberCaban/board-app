use diesel::PgConnection;
use rocket::tokio::sync::{Mutex, MutexGuard};

use crate::services;

pub struct PSQLConnection(Mutex<PgConnection>);

impl PSQLConnection {
    pub fn new() -> Self {
        Self(Mutex::new(services::connect_pg()))
    }

    pub fn get(&self) -> Result<MutexGuard<'_, PgConnection>, rocket::tokio::sync::TryLockError> {
        self.0.try_lock()
    }
}

#[macro_export]
macro_rules! connect_db {
    ($db:expr) => {
        match $db.get() {
            Ok(c) => c,
            Err(e) => return Err(ApiError::from_error(&e).to_json()),
        }
    };
}
