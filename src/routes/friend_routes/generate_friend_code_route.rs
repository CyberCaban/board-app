use chrono::{Duration, Local};

use crate::{
    database::{friend_queries::FriendQueries, Db},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        friends::FriendCode,
    },
};

use super::friend_helpers::generate_unique_code;

#[post("/code")]
pub async fn generate_friend_code(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<FriendCode>, ApiResponse> {
    let user_id = auth.unpack()?.id;

    let code = generate_unique_code(&db)
        .await
        .map_err(ApiResponse::from_error)?;
    let expires_at = (Local::now() + Duration::days(2)).naive_utc();

    FriendQueries::set_friend_code(&db, user_id, code.clone(), expires_at)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(FriendCode { code, expires_at }))
}
