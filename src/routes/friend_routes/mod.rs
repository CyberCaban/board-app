use chrono::{Duration, Local, NaiveDateTime};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;
use getrandom;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        friends::{FriendCode, FriendRelationship},
    },
    schema::{friends, users},
};

/// # POST /friends/code
/// Generate a new friend code for the user
/// # Arguments
/// * `db` - The database connection
/// * `auth` - The authentication result
/// # Returns
/// * `FriendCode` - The new friend code
#[post("/code")]
pub async fn generate_friend_code(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<FriendCode>, ApiResponse<ApiError>> {
    let user_id = auth.unpack()?.id;

    let code = generate_unique_code(&db).await?;
    let expires_at = (Local::now() + Duration::days(2)).naive_utc();

    let db_code = code.clone();
    db.run(move |conn| {
        diesel::update(users::table.find(user_id))
            .set((
                users::friend_code.eq(&db_code),
                users::friend_code_expires_at.eq(expires_at),
            ))
            .execute(conn)
    })
    .await
    .map_err(|e| ApiResponse::from_error(e.into()))?;

    Ok(ApiResponse::new(FriendCode { code, expires_at }))
}

/// # GET /friends/code
/// Get the current friend code for the user
/// # Arguments
/// * `db` - The database connection
/// * `auth` - The authentication result
/// # Returns
/// * `Option<FriendCode>` - The current friend code
#[get("/code")]
pub async fn get_friend_code(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Option<FriendCode>>, ApiResponse<ApiError>> {
    let user_id = auth.unpack()?.id;

    let result = db
        .run(move |conn| {
            users::table
                .find(user_id)
                .select((users::friend_code, users::friend_code_expires_at))
                .first::<(Option<String>, Option<NaiveDateTime>)>(conn)
        })
        .await
        .map_err(|e| ApiResponse::from_error(e.into()))?;

    Ok(ApiResponse::new(match result {
        (Some(code), Some(expires_at)) => Some(FriendCode { code, expires_at }),
        _ => None,
    }))
}

/// # POST /friends/redeem
/// Redeem a friend code
/// # Arguments
/// * `db` - The database connection
/// * `auth` - The authentication result
/// * `code` - The friend code
/// # Returns
/// * `FriendRelationship` - The new friend relationship
#[post("/redeem", data = "<code>")]
pub async fn redeem_friend_code(
    db: Db,
    auth: AuthResult,
    code: String,
) -> Result<ApiResponse<FriendRelationship>, ApiResponse<ApiError>> {
    let user_id = auth.unpack()?.id;
    let code = code.trim().replace("\"", "");

    db.run(move |conn| {
        conn.transaction(|conn| {
            // Find code owner
            let friend = users::table
                .filter(users::friend_code.eq(&code))
                // .filter(users::friend_code_expires_at.gt(Local::now().naive_utc()))
                .select((users::id, users::friend_code, users::friend_code_expires_at))
                .first::<(Uuid, Option<String>, Option<NaiveDateTime>)>(conn)?;

            // Prevent self-friending

            if friend.0 == user_id {
                return Err(ApiError::from_type(ApiErrorType::InvalidRequest));
            }

            // Create mutual friendship
            diesel::insert_into(friends::table)
                .values(&vec![
                    (
                        friends::user_id.eq(user_id),
                        friends::friend_id.eq(friend.0),
                    ),
                    (
                        friends::user_id.eq(friend.0),
                        friends::friend_id.eq(user_id),
                    ),
                ])
                .execute(conn)?;

            // Invalidate used code
            diesel::update(users::table.find(friend.0))
                .set((
                    users::friend_code.eq::<Option<String>>(None),
                    users::friend_code_expires_at.eq::<Option<NaiveDateTime>>(None),
                ))
                .execute(conn)?;

            Ok(FriendRelationship {
                user_id,
                friend_id: friend.0,
                created_at: Local::now().naive_utc(),
            })
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(e.into()))
}

/// Generate unique 8-digit alphanumeric code
async fn generate_unique_code(db: &Db) -> Result<String, ApiError> {
    let mut buffer = [0u8; 8];
    getrandom::getrandom(&mut buffer).unwrap();
    let code = buffer.iter()
        .map(|b| (b % 36) as u8)
        .map(|c| {
            if c < 10 {
                (c + b'0') as char
            } else {
                (c - 10 + b'A') as char
            }
        })
        .collect::<String>();

    let db_code = code.clone();
    let exists = db
        .run(move |conn| {
            users::table
                .filter(users::friend_code.eq(&db_code))
                .count()
                .get_result::<i64>(conn)
        })
        .await?;

    if exists == 0 {
        return Ok(code);
    }

    Err(ApiError::from_type(ApiErrorType::InvalidRequest))
}
