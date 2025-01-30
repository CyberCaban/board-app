use diesel::{result::Error, BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        BoardUsersRelation,
    },
    schema::{board_users_relation, boards},
};

/// # POST /boards/<board_id>/collaborators
/// Adds a collaborator to the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `auth` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[post("/<board_id>/collaborators", data = "<collaborator_id>")]
pub async fn boards_add_collaborator(
    db: Db,
    auth: AuthResult,
    board_id: String,
    collaborator_id: Json<Uuid>,
) -> Result<ApiResponse<Uuid>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            // Check if current user is board member
            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            // Check if current user is board creator
            let _ = boards::table
                .filter(boards::id.eq(board_id).and(boards::creator_id.eq(token)))
                .select(boards::id)
                .first::<Uuid>(conn)?;

            let collaborator = diesel::insert_into(board_users_relation::table)
                .values(BoardUsersRelation {
                    board_id,
                    user_id: collaborator_id.0,
                })
                .returning(board_users_relation::user_id)
                .get_result::<Uuid>(conn)?;

            Ok::<Uuid, Error>(collaborator)
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # GET /boards/<board_id>/collaborators
/// Returns all the collaborators of the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `auth` - Takes the token of the user
/// # Returns
/// * `collaborators` - A list of collaborators id's of the board
#[get("/<board_id>/collaborators")]
pub async fn boards_get_collaborators(
    db: Db,
    auth: AuthResult,
    board_id: String,
) -> Result<ApiResponse<Vec<Uuid>>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;

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

            Ok::<Vec<Uuid>, Error>(ids)
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # GET /boards/<board_id>/collaborators/<collaborator_id>
/// Returns the collaborator with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `collaborator_id` - The id of the collaborator
/// * `auth` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[get("/<board_id>/collaborators/<collaborator_id>")]
pub async fn boards_get_collaborator(
    db: Db,
    auth: AuthResult,
    board_id: String,
    collaborator_id: String,
) -> Result<ApiResponse<Uuid>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;

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
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(
                            Uuid::try_parse(&collaborator_id)
                                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?,
                        )),
                )
                .select(board_users_relation::user_id)
                .first::<Uuid>(conn)?;

            Ok::<Uuid, Error>(collaborator)
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # DELETE /boards/<board_id>/collaborators/<collaborator_id>
/// Removes the collaborator with the given id from the board
/// # Arguments
/// * `board_id` - The id of the board
/// * `collaborator_id` - The id of the collaborator
/// * `auth` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[delete("/<board_id>/collaborators/<collaborator_id>")]
pub async fn boards_remove_collaborator(
    db: Db,
    auth: AuthResult,
    board_id: String,
    collaborator_id: String,
) -> Result<ApiResponse<Uuid>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            // Check if current user is board creator
            let _ = boards::table
                .filter(boards::id.eq(board_id).and(boards::creator_id.eq(token)))
                .select(boards::id)
                .first::<Uuid>(conn)?;

            let collaborator_id = Uuid::try_parse(&collaborator_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            diesel::delete(
                board_users_relation::table
                    .filter(board_users_relation::board_id.eq(board_id))
                    .filter(board_users_relation::user_id.eq(collaborator_id)),
            )
            .execute(conn)?;

            Ok::<Uuid, Error>(collaborator_id)
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}
