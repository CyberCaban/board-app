use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::ApiError,
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        user::{PubUser, User},
    },
    schema::{friends, users},
};

pub mod chat;
pub mod conversations;
mod helpers;

/// # GET /friends/list
/// Returns a list of all the friends of the user
/// # Arguments
/// * auth - The token of the user
/// # Returns
/// * friends - A list of all the friends of the user
#[get("/list")]
pub async fn get_friends(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Vec<PubUser>>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let friends = db
        .run(move |conn| {
            let ids: Vec<Uuid> = friends::table
                .filter(friends::user_id.eq(token))
                .select(friends::friend_id)
                .load::<Uuid>(conn)
                .map_err(|e| ApiError::new("Failed to load friends", e.to_string()))?;
            let users: Vec<User> = users::table
                .filter(users::id.eq_any(ids))
                .select(users::all_columns)
                .load::<User>(conn)
                .map_err(|e| ApiError::new("Failed to load friends", e.to_string()))?;
            Ok::<Vec<PubUser>, ApiError>(users.into_iter().map(|user| user.into()).collect())
        })
        .await;

    match friends {
        Ok(friends) => Ok(ApiResponse::new(friends)),
        Err(e) => Err(ApiResponse::from_error(e)),
    }
}
