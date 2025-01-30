use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    http::CookieJar,
    serde::json::{json, Json},
};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        BoardColumn, BoardUsersRelation, NewColumn, PubColumn, ReturnedColumn,
    },
    schema::{board_column, board_users_relation, card_attachments, column_card, files},
};

/// # POST /boards/<board_id>/columns
/// Creates a new column in the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `auth` - Takes the token of the user
/// # Returns
/// * `column_id` - The id of the column
#[post("/<board_id>/columns", data = "<column>")]
pub async fn boards_create_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column: Json<NewColumn>,
) -> Result<ApiResponse<Uuid>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let board_id = Uuid::try_parse(&board_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

    db.run(move |conn| {
        conn.transaction(|conn| {
            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let column_id = diesel::insert_into(board_column::table)
                .values(BoardColumn {
                    id: None,
                    name: column.name.clone(),
                    board_id,
                    position: column.position,
                })
                .returning(board_column::id)
                .get_result::<Uuid>(conn)?;

            Ok::<Uuid, diesel::result::Error>(column_id)
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # GET /boards/<board_id>/columns
/// Returns all the columns of the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `auth` - Takes the token of the user
/// # Returns
/// * `columns` - A list of columns id's of the board
/// ```json
///  [
///        {
///            "id": <column_id>,
///            "name": <column_name>,
///            "position": <column_position>
///        },
///        ...
///  ]
/// ```
#[get("/<board_id>/columns")]
pub async fn boards_get_columns(
    db: Db,
    auth: AuthResult,
    board_id: String,
) -> Result<ApiResponse<Vec<PubColumn>>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let board_id = Uuid::try_parse(&board_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

    db.run(move |conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;

        let columns = board_column::table
            .filter(board_column::board_id.eq(board_id))
            .select((board_column::id, board_column::name, board_column::position))
            .load::<ReturnedColumn>(conn)?
            .into_iter()
            .map(|col| PubColumn {
                id: col.0,
                name: col.1,
                position: col.2,
            })
            .collect::<Vec<PubColumn>>();

        Ok::<Vec<PubColumn>, diesel::result::Error>(columns)
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # GET /boards/<board_id>/columns/<column_id>
/// Returns the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `auth` - Takes the token of the user
/// # Returns
/// * `column` - The column
/// ```json
/// {
///     "id": <column_id>,
///     "name": <column_name>,
///     "position": <column_position>
/// }
/// ```
#[get("/<board_id>/columns/<column_id>")]
pub async fn boards_get_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
) -> Result<ApiResponse<PubColumn>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let board_id = Uuid::try_parse(&board_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

    db.run(move |conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;

        let column = board_column::table
            .filter(board_column::id.eq(Uuid::try_parse(&column_id).map_err(|_| {
                ApiError::from_type(ApiErrorType::FailedToParseUUID)
            })?))
            .select((board_column::id, board_column::name, board_column::position))
            .first::<ReturnedColumn>(conn)?;

        Ok::<PubColumn, diesel::result::Error>(PubColumn {
            id: column.0,
            name: column.1,
            position: column.2,
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # PUT /boards/<board_id>/columns/<column_id>
/// Updates the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `auth` - Takes the token of the user
/// # Returns
/// * `column` - The column
/// ```json
/// {
///     "id": <column_id>,
///     "name": <column_name>,
///     "position": <column_position>
/// }
/// ```
#[put("/<board_id>/columns/<column_id>", data = "<column>")]
pub async fn boards_update_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    column: Json<NewColumn>,
) -> Result<ApiResponse<PubColumn>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let board_id = Uuid::try_parse(&board_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

    db.run(move |conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;

        let column = diesel::update(board_column::table)
            .filter(board_column::id.eq(Uuid::try_parse(&column_id).map_err(|_| {
                ApiError::from_type(ApiErrorType::FailedToParseUUID)
            })?))
            .set((
                board_column::name.eq(column.name.clone()),
                board_column::position.eq(column.position),
            ))
            .returning((board_column::id, board_column::name, board_column::position))
            .get_result::<ReturnedColumn>(conn)?;

        Ok::<PubColumn, diesel::result::Error>(PubColumn {
            id: column.0,
            name: column.1,
            position: column.2,
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}

/// # DELETE /boards/<board_id>/columns/<column_id>
/// Deletes the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `auth` - Takes the token of the user
/// # Returns
/// * `column` - The column
/// ```json
/// {
///     "id": <column_id>,
///     "name": <column_name>,
///     "position": <column_position>
/// }
/// ```
#[delete("/<board_id>/columns/<column_id>")]
pub async fn boards_delete_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
) -> Result<ApiResponse<PubColumn>, ApiResponse<ApiError>> {
    let token = auth.unpack()?.id;
    let board_id = Uuid::try_parse(&board_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

    db.run(move |conn| {
        conn.transaction(|conn| {
            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let cards = column_card::table
                .filter(
                    column_card::column_id.eq(Uuid::try_parse(&column_id)
                        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?),
                )
                .select((column_card::id, column_card::name))
                .load::<(Uuid, String)>(conn)?;

            for (card_id, _) in cards {
                let attachments = card_attachments::table
                    .filter(card_attachments::card_id.eq(card_id))
                    .inner_join(files::table)
                    .select((card_attachments::file_id, files::name))
                    .load::<(Uuid, String)>(conn)?;

                for (attachment, file_name) in attachments {
                    diesel::delete(card_attachments::table)
                        .filter(card_attachments::card_id.eq(card_id))
                        .filter(card_attachments::file_id.eq(attachment))
                        .execute(conn)?;
                    diesel::delete(files::table)
                        .filter(files::id.eq(attachment))
                        .execute(conn)?;
                    std::fs::remove_file(format!("tmp/{}", file_name)).unwrap();
                }
            }
            let _ = diesel::delete(column_card::table)
                .filter(
                    column_card::column_id.eq(Uuid::try_parse(&column_id)
                        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?),
                )
                .execute(conn)?;

            let column = diesel::delete(board_column::table)
                .filter(
                    board_column::id.eq(Uuid::try_parse(&column_id)
                        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?),
                )
                .returning((board_column::id, board_column::name, board_column::position))
                .get_result::<ReturnedColumn>(conn)?;

            Ok::<PubColumn, diesel::result::Error>(PubColumn {
                id: column.0,
                name: column.1,
                position: column.2,
            })
        })
    })
    .await
    .map(ApiResponse::new)
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))
}
