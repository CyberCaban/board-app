use crate::{
    database::{friend_queries::FriendQueries, Db},
    errors::{ApiError, ApiErrorType},
};

pub async fn generate_unique_code(db: &Db) -> Result<String, ApiError> {
    for _ in 0..10 {
        let mut buffer = [0u8; 8];
        getrandom::getrandom(&mut buffer).map_err(ApiError::from_error)?;

        let code = buffer
            .iter()
            .map(|b| (b % 36) as u8)
            .map(|c| {
                if c < 10 {
                    (c + b'0') as char
                } else {
                    (c - 10 + b'A') as char
                }
            })
            .collect::<String>();

        if !FriendQueries::friend_code_exists(db, code.clone()).await? {
            return Ok(code);
        }
    }

    Err(ApiError::from_type(ApiErrorType::InternalServerError))
}
