use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{http::CookieJar, serde::json::Json, State};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    check_user_token, connect_db,
    database::PSQLConnection,
    errors::{ApiError, ApiErrorType},
    models::{BoardUsersRelation, ColumnCard, NewCard, PubCard, ReturnedCard, SELECT_CARD},
    schema::{board_column, board_users_relation, column_card},
};

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
                name: card.name.clone(),
                column_id: column,
                position: card.position,
                description: card
                    .description
                    .clone()
                    .map(|description| description.to_string()),
            })
            .returning(SELECT_CARD)
            .get_result::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            name: card.1,
            cover_attachment: card.2,
            position: card.3,
            description: card.4,
            column_id: card.5
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
            .select(SELECT_CARD)
            .get_results::<ReturnedCard>(conn)?
            .into_iter()
            .map(|card| PubCard {
                id: card.0,
                name: card.1,
                cover_attachment: card.2,
                position: card.3,
                description: card.4,
                column_id: card.5,
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
        let _ = board_column::table
            .filter(board_column::id.eq(Uuid::parse_str(column_id).unwrap()))
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = column_card::table
            .filter(column_card::id.eq(Uuid::parse_str(card_id).unwrap()))
            .select(SELECT_CARD)
            .first::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            name: card.1,
            cover_attachment: card.2,
            position: card.3,
            description: card.4,
            column_id: card.5
        }))
    })
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}

/// # GET /boards/<board_id>/cards/<card_id>
/// Returns the card with the given id
/// # Arguments
/// * `board_id` - The id of the board
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
#[get("/<board_id>/cards/<card_id>")]
pub fn boards_get_card_by_id(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    card_id: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    let card_id = Uuid::try_parse(card_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)?;
        let card = column_card::table
            .filter(column_card::id.eq(card_id))
            .select(SELECT_CARD)
            .first::<ReturnedCard>(conn)?;
        let card = PubCard {
            id: card.0,
            name: card.1,
            cover_attachment: card.2,
            position: card.3,
            description: card.4,
            column_id: card.5
        };

        Ok::<PubCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(card))
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
/// * `card` - The card informatioe
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
            .returning(SELECT_CARD)
            .get_result::<ReturnedCard>(conn)?;
        Ok::<ReturnedCard, diesel::result::Error>(card)
    })
    .map(|card| {
        Json(json!(PubCard {
            id: card.0,
            name: card.1,
            cover_attachment: card.2,
            position: card.3,
            description: card.4,
            column_id: card.5
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
    let board_id = Uuid::try_parse(board_id).map_err(|_| {
        return ApiError::from_message("Failed to parse board id".to_string()).to_json();
    })?;
    let column_id = Uuid::try_parse(column_id).map_err(|_| {
        return ApiError::from_message("Failed to parse column id".to_string()).to_json();
    })?;
    let card1 = Uuid::try_parse(card1_id).map_err(|_| {
        return ApiError::from_message("Failed to parse card 1 id".to_string()).to_json();
    })?;
    let card2 = Uuid::try_parse(card2_id).map_err(|_| {
        return ApiError::from_message("Failed to parse card 2 id".to_string()).to_json();
    })?;
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

/// # PUT /boards/<board_id>/columns/<from_column_id>/cards/<card_id>/reorder
/// Reorders the cards in the columns
/// # Arguments
/// * `board_id` - The id of the board
///  * `from_column_id` - The id of the column
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `reordered_cards` - The reordered cards
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
/// ```
#[put("/<board_id>/columns/<from_column_id>/cards/<card_id>/reorder/<to_column_id>/<to_pos>")]
pub fn boards_reorder_cards(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
    board_id: &str,
    from_column_id: &str,
    card_id: &str,
    to_column_id: &str,
    to_pos: &str,
) -> Result<Json<Value>, Json<Value>> {
    let mut conn = connect_db!(db);
    let token = check_user_token!(cookies, conn);
    let conn = &mut *conn;
    let board_id = Uuid::try_parse(board_id).map_err(|_| {
        return ApiError::from_message("Failed to parse board id".to_string()).to_json();
    })?;
    let from_column_id = Uuid::try_parse(from_column_id).map_err(|_| {
        return ApiError::from_message("Failed to parse from column id".to_string()).to_json();
    })?;
    let card_id = Uuid::try_parse(card_id).map_err(|_| {
        return ApiError::from_message("Failed to parse card id".to_string()).to_json();
    })?;
    let to_column_id = Uuid::try_parse(to_column_id).map_err(|_| {
        return ApiError::from_message("Failed to parse to column id".to_string()).to_json();
    })?;
    let to_pos = to_pos.parse::<i32>().map_err(|_| {
        return ApiError::from_message("Failed to parse to position".to_string()).to_json();
    })?;
    conn.transaction(|conn| {
        let _ = board_users_relation::table
            .filter(
                board_users_relation::board_id
                    .eq(board_id)
                    .and(board_users_relation::user_id.eq(token)),
            )
            .first::<BoardUsersRelation>(conn)
            .map_err(|_| {
                ApiError::from_message("You are not a member of this board".to_string()).to_json()
            });

        let card = column_card::table
            .filter(
                column_card::id
                    .eq(card_id)
                    .and(column_card::column_id.eq(from_column_id)),
            )
            .select((column_card::id, column_card::position))
            .first::<(Uuid, i32)>(conn)?;
        let from_column = board_column::table
            .filter(board_column::id.eq(from_column_id))
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let to_column = board_column::table
            .filter(board_column::id.eq(to_column_id))
            .select(board_column::id)
            .first::<Uuid>(conn)?;

        if from_column == to_column {
            if card.1 < to_pos {
                diesel::update(
                    column_card::table.filter(
                        column_card::column_id
                            .eq(from_column)
                            .ne(column_card::id.eq(card_id))
                            .and(column_card::position.between(card.1, to_pos)),
                    ),
                )
                .set(column_card::position.eq(column_card::position - 1))
                .execute(conn)?;
            } else if card.1 > to_pos {
                diesel::update(
                    column_card::table.filter(
                        column_card::column_id
                            .eq(from_column)
                            .ne(column_card::id.eq(card_id))
                            .and(column_card::position.between(to_pos, card.1)),
                    ),
                )
                .set(column_card::position.eq(column_card::position + 1))
                .execute(conn)?;
            }
        } else {
            diesel::update(
                column_card::table.filter(
                    column_card::column_id
                        .eq(from_column)
                        .ne(column_card::id.eq(card_id))
                        .and(column_card::position.gt(card.1)),
                ),
            )
            .set(column_card::position.eq(column_card::position - 1))
            .execute(conn)?;
            diesel::update(
                column_card::table.filter(
                    column_card::column_id
                        .eq(to_column)
                        .ne(column_card::id.eq(card_id))
                        .and(column_card::position.ge(to_pos)),
                ),
            )
            .set(column_card::position.eq(column_card::position + 1))
            .execute(conn)?;
        }

        diesel::update(column_card::table.filter(column_card::id.eq(card.0)))
            .set((
                column_card::column_id.eq(to_column),
                column_card::position.eq(to_pos),
            ))
            .execute(conn)?;
        Ok::<Uuid, diesel::result::Error>(card.0)
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
    let column_id = Uuid::try_parse(column_id)
        .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
    let card_id = Uuid::try_parse(card_id)
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
            .filter(board_column::id.eq(column_id))
            .select(board_column::id)
            .first::<Uuid>(conn)?;
        let card = column_card::table
            .filter(column_card::id.eq(card_id))
            .select((column_card::id, column_card::position))
            .first::<(Uuid, i32)>(conn)?;
        diesel::update(
            column_card::table.filter(
                column_card::column_id
                    .eq(column_id)
                    .and(column_card::position.gt(card.1)),
            ),
        )
        .set(column_card::position.eq(column_card::position - 1))
        .execute(conn)?;

        let card = diesel::delete(column_card::table)
            .filter(
                column_card::id
                    .eq(card_id)
                    .and(column_card::column_id.eq(column)),
            )
            .returning(column_card::id)
            .get_result::<Uuid>(conn)?;
        Ok::<Uuid, diesel::result::Error>(card)
    })
    .map(|card| (Json(json!(card))))
    .map_err(|e| (ApiError::from_error(&e).to_json()))
}
