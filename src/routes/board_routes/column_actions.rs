use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::CookieJar, serde::json::Json, State};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    check_user_token, connect_db,
    database::PSQLConnection,
    errors::{ApiError, ApiErrorType},
    models::{
        BoardColumn, BoardUsersRelation, NewColumn, PubColumn, ReturnedColumn,
    },
    schema::{board_column, board_users_relation, column_card},
};

/// # POST /boards/<board_id>/columns
/// Creates a new column in the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `column_id` - The id of the column
#[post("/<board_id>/columns", data = "<column>")]
pub fn boards_create_column(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column: Json<NewColumn>,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let column = diesel::insert_into(board_column::table)
            .values(BoardColumn {
                id: None,
                name: column.name.map(|name| name.to_string()),
                board_id,
                position: column.position,
            })
            .returning(board_column::id)
            .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(column)
    })
    .map(|id| (Json(json!(id))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/columns
/// Returns all the columns of the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
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
pub fn boards_get_columns(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
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
    .map(|columns| (Json(json!(columns))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/columns/<column_id>
/// Returns the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `cookies` - Takes the token of the user
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
pub fn boards_get_column(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let column = board_column::table
            .filter(board_column::id.eq(Uuid::parse_str(column_id).unwrap()))
            .select((board_column::id, board_column::name, board_column::position))
            .first::<ReturnedColumn>(conn)?;
        Ok::<ReturnedColumn, diesel::result::Error>(column)
    })
    .map(|column| {
        Json(json!(PubColumn {
            id: column.0,
            name: column.1,
            position: column.2
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # PUT /boards/<board_id>/columns/<column_id>
/// Updates the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `cookies` - Takes the token of the user
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
pub fn boards_update_column(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    column: Json<NewColumn>,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let column = diesel::update(board_column::table)
            .filter(board_column::id.eq(Uuid::parse_str(column_id).unwrap()))
            .set((
                board_column::name.eq(column.name.map(|name| name.to_string())),
                board_column::position.eq(column.position),
            ))
            .returning((board_column::id, board_column::name, board_column::position))
            .get_result::<ReturnedColumn>(conn)?;
        Ok::<ReturnedColumn, diesel::result::Error>(column)
    })
    .map(|column| {
        Json(json!(PubColumn {
            id: column.0,
            name: column.1,
            position: column.2
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # DELETE /boards/<board_id>/columns/<column_id>
/// Deletes the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `cookies` - Takes the token of the user
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
pub fn boards_delete_column(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let _ = diesel::delete(column_card::table)
            .filter(column_card::column_id.eq(Uuid::parse_str(column_id).unwrap()))
            .execute(conn)?;
        let column = diesel::delete(board_column::table)
            .filter(board_column::id.eq(Uuid::parse_str(column_id).unwrap()))
            .returning((board_column::id, board_column::name, board_column::position))
            .get_result::<ReturnedColumn>(conn)?;
        Ok::<ReturnedColumn, diesel::result::Error>(column)
    })
    .map(|column| {
        Json(json!(PubColumn {
            id: column.0,
            name: column.1,
            position: column.2
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}
