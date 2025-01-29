use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, PgArrayExpressionMethods, QueryDsl,
    RunQueryDsl,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{api_response::ApiResponse, auth::AuthResult, FriendsRequest},
    schema::{friends_requests, users},
};

mod helpers;

/// # POST /friends/send/<user>
/// Sends a friend request
/// # Arguments
/// * user - The id of the user you want to friend
/// * auth - The token of the user
/// # Returns
/// * friend_request_id - The id of the friend request
#[post("/send/<user>")]
pub async fn frend_request_send(
    db: Db,
    auth: AuthResult,
    user: String,
) -> Result<ApiResponse<String>, ApiResponse> {
    let user = Uuid::parse_str(&user).map_err(|_| {
        ApiResponse::from_error(ApiError::new("InvalidUserId", ApiErrorType::InvalidUserId))
    })?;
    let token = auth.unpack()?.id;
    let tr = db
        .run(move |conn| {
            conn.transaction(|conn| {
                let is_friends = users::table
                    .filter(
                        users::id.eq_any(vec![token, user]).and(
                            users::friends
                                .contains(vec![user])
                                .or(users::friends.contains(vec![token])),
                        ),
                    )
                    .count()
                    .get_result::<i64>(conn);
                if is_friends? > 0 {
                    return Err(ApiError::new(
                        "AlreadyFriends",
                        ApiErrorType::AlreadyFriends,
                    ));
                }
                let id = helpers::add_request_to_db(conn, token, user)?;
                Ok(id)
            })
        })
        .await;
    match tr {
        Err(e) => Err(ApiResponse::from_error(e.into())),
        Ok(id) => Ok(ApiResponse::new(id.to_string())),
    }
}

/// # DELETE /friends/cancel/<user_id>
/// Cancels a friend request
/// # Arguments
/// * user_id - The id of the user you want to cancel the friend request
/// * auth - The token of the user
/// # Returns
/// * Ok - If the request was cancelled
#[delete("/cancel/<user_id>")]
pub async fn frend_request_cancel(
    db: Db,
    auth: AuthResult,
    user_id: String,
) -> Result<ApiResponse<String>, ApiResponse> {
    let user_id = Uuid::parse_str(&user_id).map_err(|_| {
        ApiResponse::from_error(ApiError::new("InvalidUserId", ApiErrorType::InvalidUserId))
    })?;
    let token = auth.unpack()?.id;
    let tr = db
        .run(move |conn| helpers::delete_request(conn, token, user_id))
        .await;
    match tr {
        Err(e) => Err(ApiResponse::from_error(e.into())),
        Ok(_) => Ok(ApiResponse::new("ok".to_string())),
    }
}

/// # GET /friends
/// Returns a list of all the friend requests of the user
/// # Arguments
/// * auth - The token of the user
/// # Returns
/// * requests - A list of all the friend requests of the user
/// ```json
/// [
///     {
///         "id": <request_id>,
///         "sender_id": <sender_id>,
///         "receiver_id": <receiver_id>,
///         "created_at": <created_at>,
///         "updated_at": <updated_at>,
///     },
///     ...
/// ]
/// ```
#[get("/")]
pub async fn frend_requests_list(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Value>, ApiResponse> {
    let token = auth.unpack()?.id;
    match db
        .run(move |conn| helpers::load_user_requests(conn, token))
        .await
    {
        Ok(requests) => Ok(ApiResponse::new(json!(requests))),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

/// # POST /friends/accept/<request_id>
/// Accepts a friend request
/// # Arguments
/// * request_id - The id of the friend request
/// * auth - The token of the user
/// # Returns
/// * Ok - If the request was accepted
#[post("/accept/<request_id>")]
pub async fn frend_request_accept(
    db: Db,
    auth: AuthResult,
    request_id: String,
) -> Result<ApiResponse<String>, ApiResponse> {
    let token = auth.unpack()?.id;
    let request_id = Uuid::parse_str(&request_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
    match db
        .run(move |conn| {
            conn.transaction(|conn| {
                let FriendsRequest {
                    id,
                    sender_id,
                    receiver_id,
                    ..
                } = friends_requests::table
                    .filter(friends_requests::receiver_id.eq(token))
                    .first::<FriendsRequest>(conn)?;
                if id != request_id {
                    return Err(ApiError::from_type(ApiErrorType::InvalidRequest));
                }
                diesel::delete(
                    friends_requests::table.filter(friends_requests::receiver_id.eq(token)),
                )
                .execute(conn)?;
                let receiver_friends = helpers::get_user_friends(conn, receiver_id)?;
                let mut receiver_friends = receiver_friends.unwrap_or(vec![]);
                receiver_friends.push(Some(sender_id));
                let sender_friends = helpers::get_user_friends(conn, sender_id)?;
                let mut sender_friends = sender_friends.unwrap_or(vec![]);
                sender_friends.push(Some(receiver_id));

                helpers::set_user_friends(conn, receiver_id, Some(receiver_friends))?;
                helpers::set_user_friends(conn, sender_id, Some(sender_friends))?;
                Ok::<(), ApiError>(())
            })
        })
        .await
    {
        Ok(()) => Ok(ApiResponse::new("Ok".to_string())),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

/// # DELETE /friends/decline/<request_id>
/// Declines a friend request
/// # Arguments
/// * request_id - The id of the friend request
/// * auth - The token of the user
/// # Returns
/// * Ok - If the request was declined
#[delete("/decline/<request_id>")]
pub async fn frend_request_decline(
    db: Db,
    auth: AuthResult,
    request_id: String,
) -> Result<ApiResponse<String>, ApiResponse> {
    let token = auth.unpack()?.id;
    let request_id = Uuid::parse_str(&request_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
    match db
        .run(move |conn| {
            diesel::delete(
                friends_requests::table.filter(
                    friends_requests::receiver_id
                        .eq(token)
                        .and(friends_requests::id.eq(request_id)),
                ),
            )
            .execute(conn)
        })
        .await
    {
        Ok(_) => Ok(ApiResponse::new("Ok".to_string())),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
