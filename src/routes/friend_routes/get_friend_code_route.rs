use crate::{
    database::{friend_queries::FriendQueries, Db},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        friends::FriendCode,
    },
};

#[get("/code")]
pub async fn get_friend_code(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Option<FriendCode>>, ApiResponse> {
    let user_id = auth.unpack()?.id;

    let result = FriendQueries::get_friend_code(&db, user_id)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(match result {
        (Some(code), Some(expires_at)) => Some(FriendCode { code, expires_at }),
        _ => None,
    }))
}
