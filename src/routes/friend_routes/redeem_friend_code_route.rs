use crate::{
    database::{friend_queries::FriendQueries, Db},
    errors::ApiErrorType,
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        friends::FriendRelationship,
    },
};

#[post("/redeem", data = "<code>")]
pub async fn redeem_friend_code(
    db: Db,
    auth: AuthResult,
    code: String,
) -> Result<ApiResponse<FriendRelationship>, ApiResponse> {
    let user_id = auth.unpack()?.id;
    let sanitized_code = code.trim().replace('"', "");

    if sanitized_code.is_empty() {
        return Err(ApiResponse::from_error_type(ApiErrorType::InvalidRequest));
    }

    let relation = FriendQueries::redeem_friend_code(&db, user_id, sanitized_code)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(relation))
}
