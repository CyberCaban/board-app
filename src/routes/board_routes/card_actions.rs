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
        BoardUsersRelation, CardInfo, ColumnCard, NewCard, PubCard, ReturnedCard, SELECT_CARD,
    },
    schema::{board_column, board_users_relation, column_card},
    validate_user_token,
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
pub async fn boards_create_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    column_id: String,
    card: Json<NewCard>,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let column_id = Uuid::try_parse(&column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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

            let card = diesel::insert_into(column_card::table)
                .values(ColumnCard {
                    id: None,
                    name: card.name.clone(),
                    column_id: column,
                    position: card.position,
                    description: card.description.clone(),
                })
                .returning(SELECT_CARD)
                .get_result::<ReturnedCard>(conn)?;

            Ok::<ReturnedCard, diesel::result::Error>(card)
        })
    })
    .await
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
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_get_cards(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    column_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let column_id = Uuid::try_parse(&column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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
    })
    .await
    .map(|cards| Json(json!(cards)))
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_get_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    column_id: String,
    card_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let column_id = Uuid::try_parse(&column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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
                .filter(column_card::column_id.eq(column))
                .select(SELECT_CARD)
                .first::<ReturnedCard>(conn)?;

            Ok::<ReturnedCard, diesel::result::Error>(card)
        })
    })
    .await
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
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_get_card_by_id(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    card_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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

            Ok::<ReturnedCard, diesel::result::Error>(card)
        })
    })
    .await
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
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_update_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    column_id: String,
    card_id: String,
    card: Json<CardInfo>,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let column_id = Uuid::try_parse(&column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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

            let card = diesel::update(column_card::table)
                .filter(column_card::id.eq(card_id))
                .filter(column_card::column_id.eq(column))
                .set((
                    column_card::name.eq(card.name.clone()),
                    column_card::description.eq(card.description.clone()),
                ))
                .returning(SELECT_CARD)
                .get_result::<ReturnedCard>(conn)?;

            Ok::<ReturnedCard, diesel::result::Error>(card)
        })
    })
    .await
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
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_reorder_cards(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    from_column_id: String,
    card_id: String,
    to_column_id: String,
    to_pos: i32,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let from_column_id = Uuid::try_parse(&from_column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let to_column_id = Uuid::try_parse(&to_column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let (card_id, pos) = column_card::table
                .filter(column_card::id.eq(card_id))
                .filter(column_card::column_id.eq(from_column_id))
                .select((column_card::id, column_card::position))
                .first::<(Uuid, i32)>(conn)?;

            // Update positions in the source column
            diesel::update(column_card::table)
                .filter(column_card::column_id.eq(from_column_id))
                .filter(column_card::position.gt(pos))
                .set(column_card::position.eq(column_card::position - 1))
                .execute(conn)?;

            // Update positions in the target column
            diesel::update(column_card::table)
                .filter(column_card::column_id.eq(to_column_id))
                .filter(column_card::position.ge(to_pos))
                .set(column_card::position.eq(column_card::position + 1))
                .execute(conn)?;

            // Move the card
            let card = diesel::update(column_card::table)
                .filter(column_card::id.eq(card_id))
                .set((
                    column_card::column_id.eq(to_column_id),
                    column_card::position.eq(to_pos),
                ))
                .returning(SELECT_CARD)
                .get_result::<ReturnedCard>(conn)?;

            Ok::<ReturnedCard, diesel::result::Error>(card)
        })
    })
    .await
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
    .map_err(|e| ApiError::from_error(e).to_json())
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
pub async fn boards_delete_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    column_id: String,
    card_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let column_id = Uuid::try_parse(&column_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

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

            let (card_id, pos) = column_card::table
                .filter(column_card::id.eq(card_id))
                .select((column_card::id, column_card::position))
                .first::<(Uuid, i32)>(conn)?;

            diesel::update(column_card::table)
                .filter(
                    column_card::column_id
                        .eq(column)
                        .and(column_card::position.gt(pos)),
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
    })
    .await
    .map(|card| Json(json!(card)))
    .map_err(|e| ApiError::from_error(e).to_json())
}
