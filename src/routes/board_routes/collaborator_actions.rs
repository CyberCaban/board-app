use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::CookieJar, serde::json::{json, Json}};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    validate_user_token,
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::BoardUsersRelation,
    schema::{board_users_relation, boards},
};

/// # POST /boards/<board_id>/collaborators
/// Adds a collaborator to the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[post("/<board_id>/collaborators", data = "<collaborator_id>")]
pub async fn boards_add_collaborator(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    collaborator_id: Json<Uuid>,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let collaborator = diesel::insert_into(board_users_relation::table)
                .values(BoardUsersRelation {
                    board_id,
                    user_id: collaborator_id.0,
                })
                .returning(board_users_relation::user_id)
                .get_result::<Uuid>(conn)?;

            Ok::<Uuid, diesel::result::Error>(collaborator)
        })
    })
    .await
    .map(|collaborator| Json(json!(collaborator)))
    .map_err(|e| ApiError::from_error(e).to_json())
}

/// # GET /boards/<board_id>/collaborators
/// Returns all the collaborators of the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborators` - A list of collaborators id's of the board
#[get("/<board_id>/collaborators")]
pub async fn boards_get_collaborators(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let ids = board_users_relation::table
                .filter(board_users_relation::board_id.eq(board_id))
                .select(board_users_relation::user_id)
                .load::<Uuid>(conn)?;

            Ok::<Vec<Uuid>, diesel::result::Error>(ids)
        })
    })
    .await
    .map(|collaborators| Json(json!(collaborators)))
    .map_err(|e| ApiError::from_error(e).to_json())
}

/// # GET /boards/<board_id>/collaborators/<collaborator_id>
/// Returns the collaborator with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `collaborator_id` - The id of the collaborator
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[get("/<board_id>/collaborators/<collaborator_id>")]
pub async fn boards_get_collaborator(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    collaborator_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let collaborator = board_users_relation::table
                .filter(board_users_relation::board_id.eq(board_id).and(
                    board_users_relation::user_id.eq(Uuid::try_parse(&collaborator_id)
                        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?),
                ))
                .select(board_users_relation::user_id)
                .first::<Uuid>(conn)?;

            Ok::<Uuid, diesel::result::Error>(collaborator)
        })
    })
    .await
    .map(|collaborator| Json(json!(collaborator)))
    .map_err(|e| ApiError::from_error(e).to_json())
}

/// # DELETE /boards/<board_id>/collaborators/<collaborator_id>
/// Removes the collaborator with the given id from the board
/// # Arguments
/// * `board_id` - The id of the board
/// * `collaborator_id` - The id of the collaborator
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[delete("/<board_id>/collaborators/<collaborator_id>")]
pub async fn boards_remove_collaborator(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    collaborator_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            // Check if the user is the member of the board
            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            // Check if the user is the creator of the board
            let _ = boards::table
                .filter(boards::id.eq(board_id).and(boards::creator_id.eq(token)))
                .select(boards::id)
                .first::<Uuid>(conn)?;

            let collaborator_id = Uuid::try_parse(&collaborator_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            // Delete the collaborator
            diesel::delete(board_users_relation::table)
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(collaborator_id)),
                )
                .execute(conn)?;

            Ok::<Uuid, diesel::result::Error>(collaborator_id)
        })
    })
    .await
    .map(|collaborator_id| Json(json!(collaborator_id)))
    .map_err(|e| ApiError::from_error(e).to_json())
}
