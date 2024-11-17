use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::CookieJar, serde::json::Json, State};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    check_user_token, connect_db,
    database::PSQLConnection,
    errors::{ApiError, ApiErrorType},
    models::{
        Board, BoardColumn, BoardInfo, BoardUsersRelation, ColumnCard, NewBoard, NewCard,
        NewColumn, PubBoard, PubCard, PubColumn, ReturnedCard, ReturnedColumn, SwapCards,
    },
    schema::{board_column, board_users_relation, boards, column_card},
};

/// # POST /boards
/// Creates a new board
/// # Arguments
/// * `board` - The name of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `board_id` - The id of the board
#[post("/", data = "<board>")]
pub fn boards_create_board_and_relation(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board: Json<NewBoard>,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    conn.transaction(|conn| {
        let board_id = diesel::insert_into(boards::table)
            .values(Board {
                id: None,
                name: board.name.to_string(),
                creator_id: token,
            })
            .returning(boards::id)
            .get_result::<Uuid>(conn)?;
        let _ = diesel::insert_into(board_users_relation::table)
            .values(BoardUsersRelation {
                user_id: token,
                board_id,
            })
            .execute(conn)?;
        Ok::<uuid::Uuid, diesel::result::Error>(board_id)
    })
    .map(|id| (Json(json!(id))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards
/// Returns all the boards of the user
/// # Arguments
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `boards` - A list of board id's of the user
/// ```json
/// [
///     {
///         "id": <board_id>,
///         "name": <board_name>,
///     },
///     ...
/// ]
/// ```
#[get("/")]
pub fn boards_get_boards(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    conn.transaction(|conn| {
        let ids = board_users_relation::table
            .filter(board_users_relation::user_id.eq(token))
            .select(board_users_relation::board_id)
            .load::<Uuid>(&mut *conn)?;
        let bds = ids
            .iter()
            .map(|id| {
                boards::table
                    .filter(boards::id.eq(id))
                    .select((boards::id, boards::name))
                    .first::<(Uuid, String)>(conn)
                    .map(|(id, name)| PubBoard { id, name })
            })
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Ok::<Vec<PubBoard>, diesel::result::Error>(bds)
    })
    .map(|ids| (Json(json!(ids))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>
/// Returns the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `board` - The board
/// ```json
/// {
///     "id": <board_id>,
///     "name": <board_name>,
///     "columns": [
///         {
///             "id": <column_id>,
///             "name": <column_name>,
///             "position": <column_position>
///         },
///         ...
///     ],
///     "cards": [
///         {
///             "id": <card_id>,
///             "column_id": <column_id>,
///             "description": <card_description>,
///             "position": <card_position>
///         },
///         ...
///     ]
/// }
/// ```
#[get("/<board_id>")]
pub fn boards_get_board(
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
        let board_name = boards::table
            .filter(boards::id.eq(board_id))
            .select(boards::name)
            .first::<String>(conn)?;
        let columns = board_column::table
            .filter(board_column::board_id.eq(board_id))
            .select((board_column::id, board_column::name, board_column::position))
            .load::<ReturnedColumn>(conn)?
            .into_iter()
            .map(|column| PubColumn {
                id: column.0,
                name: column.1,
                position: column.2,
            })
            .collect::<Vec<PubColumn>>();
        let cards = column_card::table
            .filter(column_card::column_id.eq_any(columns.iter().map(|column| column.id)))
            .select((
                column_card::id,
                column_card::column_id,
                column_card::description,
                column_card::position,
            ))
            .load::<ReturnedCard>(conn)?
            .into_iter()
            .map(|card| PubCard {
                id: card.0,
                column_id: card.1,
                description: card.2,
                position: card.3,
            })
            .collect::<Vec<PubCard>>();
        let board = BoardInfo {
            name: board_name,
            id: board_id,
            columns,
            cards,
        };
        Ok::<BoardInfo, diesel::result::Error>(board)
    })
    .map(|board| (Json(json!(board))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # PUT /boards/<board_id>
/// Updates the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `board_id` - The id of the board
#[put("/<board_id>", data = "<board>")]
pub fn boards_update_board(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    board: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let board = diesel::update(
            boards::table.filter(boards::id.eq(board_id).and(boards::creator_id.eq(token))),
        )
        .set(boards::name.eq(board))
        .returning(boards::id)
        .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(board)
    })
    .map(|id| (Json(json!(id))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # DELETE /boards/<board_id>
/// Deletes the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `board_id` - The id of the board
#[delete("/<board_id>")]
pub fn boards_delete_board(
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
        let column_ids = board_column::table
            .filter(board_column::board_id.eq(board_id))
            .select(board_column::id)
            .load::<Uuid>(conn)?;
        diesel::delete(column_card::table.filter(column_card::column_id.eq_any(column_ids)))
            .execute(conn)?;
        diesel::delete(board_column::table.filter(board_column::board_id.eq(board_id)))
            .execute(conn)?;
        diesel::delete(
            board_users_relation::table.filter(board_users_relation::board_id.eq(board_id)),
        )
        .execute(conn)?;
        let id = diesel::delete(
            boards::table.filter(boards::id.eq(board_id).and(boards::creator_id.eq(token))),
        )
        .returning(boards::id)
        .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(id)
    })
    .map(|id| (Json(json!(id))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

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

/// # POST /boards/<board_id>/columns/<column_id>/cards
/// Creates a new card in the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `cookies` - Takes the token of the user
/// * `card` - The card information
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[post("/<board_id>/columns/<column_id>/cards", data = "<card>")]
pub fn boards_create_card(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    card: Json<NewCard>,
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
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = diesel::insert_into(column_card::table)
            .values(ColumnCard {
                id: None,
                column_id: column,
                position: card.position,
                description: card
                    .description
                    .clone()
                    .map(|description| description.to_string()),
            })
            .returning((
                column_card::id,
                column_card::column_id,
                column_card::description,
                column_card::position,
            ))
            .get_result::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            column_id: card.1,
            description: card.2,
            position: card.3
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/columns/<column_id>/cards
/// Returns all the cards in the column with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `cards` - A list of cards in the column
/// ```json
/// [
///     {
///         "id": <card_id>,
///         "column_id": <column_id>,
///         "description": <card_description>,
///         "position": <card_position>
///     },
///     ...
/// ]
#[get("/<board_id>/columns/<column_id>/cards")]
pub fn boards_get_cards(
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
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let cards = column_card::table
            .filter(column_card::column_id.eq(column))
            .select((
                column_card::id,
                column_card::column_id,
                column_card::description,
                column_card::position,
            ))
            .get_results::<ReturnedCard>(conn)?
            .into_iter()
            .map(|card| PubCard {
                id: card.0,
                column_id: card.1,
                description: card.2,
                position: card.3,
            })
            .collect::<Vec<PubCard>>();
        Ok::<Vec<PubCard>, diesel::result::Error>(cards)
    })
    .map(|cards| (Json(json!(cards))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/columns/<column_id>/cards/<card_id>
/// Returns the card with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[get("/<board_id>/columns/<column_id>/cards/<card_id>")]
pub fn boards_get_card(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    card_id: &str,
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
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = column_card::table
            .filter(column_card::id.eq(Uuid::parse_str(card_id).unwrap()))
            .select((
                column_card::id,
                column_card::column_id,
                column_card::description,
                column_card::position,
            ))
            .first::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            column_id: card.1,
            description: card.2,
            position: card.3
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # PUT /boards/<board_id>/columns/<column_id>/cards/<card_id>
/// Updates the card with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// * `card` - The card information
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[put("/<board_id>/columns/<column_id>/cards/<card_id>", data = "<card>")]
pub fn boards_update_card(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    card_id: &str,
    card: Json<NewCard>,
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
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = diesel::update(column_card::table)
            .filter(column_card::id.eq(Uuid::parse_str(card_id).unwrap()))
            .set((
                column_card::column_id.eq(column),
                column_card::description.eq(card.description.clone()),
                column_card::position.eq(card.position),
            ))
            .returning((
                column_card::id,
                column_card::column_id,
                column_card::description,
                column_card::position,
            ))
            .get_result::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            column_id: card.1,
            description: card.2,
            position: card.3
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # PUT /boards/<board_id>/columns/<column_id>/cards/<card1_id>/<card2_id>
/// Swaps the position of two cards
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// * `card` - The card information
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[put("/<board_id>/columns/<column_id>/cards/<card1_id>/<card2_id>")]
pub fn boards_swap_card(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    card1_id: &str,
    card2_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_message("Failed to parse board id".to_string()).to_json())?;
    let column_id = Uuid::try_parse(column_id)
        .map_err(|_| return ApiError::from_message("Failed to parse column id".to_string()).to_json())?;
    let card1 = Uuid::try_parse(card1_id)
        .map_err(|_| return ApiError::from_message("Failed to parse card 1 id".to_string()).to_json())?;
    let card2 = Uuid::try_parse(card2_id)
        .map_err(|_| return ApiError::from_message("Failed to parse card 2 id".to_string()).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let column = board_column::table
            .filter(board_column::id.eq(column_id))
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card1 = column_card::table
            .filter(
                column_card::id
                    .eq(card1)
                    .and(column_card::column_id.eq(column)),
            )
            .select((column_card::id, column_card::position))
            .first::<(Uuid, i32)>(conn)?;
        let card2 = column_card::table
            .filter(
                column_card::id
                    .eq(card2)
                    .and(column_card::column_id.eq(column)),
            )
            .select((column_card::id, column_card::position))
            .first::<(Uuid, i32)>(conn)?;

        diesel::update(column_card::table)
            .filter(column_card::id.eq(card1.0))
            .set(column_card::position.eq(card2.1))
            .execute(conn)?;
        diesel::update(column_card::table)
            .filter(column_card::id.eq(card2.0))
            .set(column_card::position.eq(card1.1))
            .execute(conn)?;

        Ok::<(Uuid, Uuid), diesel::result::Error>((card1.0, card2.0))
    })
    .map(|cards| Json(json!(cards)))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # DELETE /boards/<board_id>/columns/<column_id>/cards/<card_id>
/// Deletes the card with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `column_id` - The id of the column
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `card_id` - card id
#[delete("/<board_id>/columns/<column_id>/cards/<card_id>")]
pub fn boards_delete_card(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    column_id: &str,
    card_id: &str,
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
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = diesel::delete(column_card::table)
            .filter(
                column_card::id
                    .eq(Uuid::parse_str(card_id).unwrap())
                    .and(column_card::column_id.eq(column)),
            )
            .returning(column_card::id)
            .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(card)
    })
    .map(|card| (Json(json!(card))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # POST /boards/<board_id>/collaborators
/// Adds a collaborator to the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborator_id` - The id of the collaborator
#[post("/<board_id>/collaborators", data = "<collaborator_id>")]
pub fn boards_add_collaborator(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    collaborator_id: Json<Uuid>,
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
        let collaborator = diesel::insert_into(board_users_relation::table)
            .values(BoardUsersRelation {
                board_id: board_id,
                user_id: collaborator_id.0,
            })
            .returning(board_users_relation::user_id)
            .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(collaborator)
    })
    .map(|collaborator| (Json(json!(collaborator))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/collaborators
/// Returns all the collaborators of the board with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `collaborators` - A list of collaborators id's of the board
#[get("/<board_id>/collaborators")]
pub fn boards_get_collaborators(
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
        let ids = board_users_relation::table
            .filter(board_users_relation::board_id.eq(board_id))
            .select(board_users_relation::user_id)
            .load::<Uuid>(conn)?;
        Ok::<Vec<Uuid>, diesel::result::Error>(ids)
    })
    .map(|collaborators| (Json(json!(collaborators))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
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
pub fn boards_get_collaborator(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    collaborator_id: &str,
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
        let collaborator =
            board_users_relation::table
                .filter(board_users_relation::board_id.eq(board_id).and(
                    board_users_relation::user_id.eq(Uuid::parse_str(collaborator_id).unwrap()),
                ))
                .select(board_users_relation::user_id)
                .first::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(collaborator)
    })
    .map(|collaborator| (Json(json!(collaborator))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
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
pub fn boards_remove_collaborator(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    collaborator_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
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
        let collaborator_id = Uuid::parse_str(collaborator_id).unwrap();
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
    .map(|collaborator_id| (Json(json!(collaborator_id))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}
