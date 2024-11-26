// use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
// use rocket::{
//     http::CookieJar,
//     serde::json::{json, Json},
//     State,
// };
// use serde_json::Value;
// use uuid::Uuid;

// use crate::{
//     check_user_token, connect_db,
//     database::PgConnection,
//     errors::{ApiError, ApiErrorType},
//     models::{
//         Board, BoardInfo, BoardUsersRelation, NewBoard, PubBoard, PubCard, PubColumn, ReturnedCard,
//         ReturnedColumn,
//     },
//     schema::{board_column, board_users_relation, boards, column_card},
// };

// /// # POST /boards
// /// Creates a new board
// /// # Arguments
// /// * `board` - The name of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `board_id` - The id of the board
// #[post("/", data = "<board>")]
// pub fn boards_create_board_and_relation(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board: Json<NewBoard>,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
    // let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     conn.transaction(|conn| {
//         let board_id = diesel::insert_into(boards::table)
//             .values(Board {
//                 id: None,
//                 name: board.name.to_string(),
//                 creator_id: token,
//             })
//             .returning(boards::id)
//             .get_result::<Uuid>(conn)?;
//         let _ = diesel::insert_into(board_users_relation::table)
//             .values(BoardUsersRelation {
//                 user_id: token,
//                 board_id,
//             })
//             .execute(conn)?;
//         Ok::<uuid::Uuid, diesel::result::Error>(board_id)
//     })
//     .map(|id| (Json(json!(id))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # GET /boards
// /// Returns all the boards of the user
// /// # Arguments
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `boards` - A list of board id's of the user
// /// ```json
// /// [
// ///     {
// ///         "id": <board_id>,
// ///         "name": <board_name>,
// ///     },
// ///     ...
// /// ]
// /// ```
// #[get("/")]
// pub fn boards_get_boards(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
//     let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     conn.transaction(|conn| {
//         let ids = board_users_relation::table
//             .filter(board_users_relation::user_id.eq(token))
//             .select(board_users_relation::board_id)
//             .load::<Uuid>(&mut *conn)?;
//         let bds = ids
//             .iter()
//             .map(|id| {
//                 boards::table
//                     .filter(boards::id.eq(id))
//                     .select((boards::id, boards::name))
//                     .first::<(Uuid, String)>(conn)
//                     .map(|(id, name)| PubBoard { id, name })
//             })
//             .filter_map(Result::ok)
//             .collect::<Vec<_>>();
//         Ok::<Vec<PubBoard>, diesel::result::Error>(bds)
//     })
//     .map(|ids| (Json(json!(ids))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # GET /boards/<board_id>
// /// Returns the board with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `board` - The board
// /// ```json
// /// {
// ///     "id": <board_id>,
// ///     "name": <board_name>,
// ///     "columns": [
// ///         {
// ///             "id": <column_id>,
// ///             "name": <column_name>,
// ///             "position": <column_position>
// ///         },
// ///         ...
// ///     ],
// ///     "cards": [
// ///         {
// ///             "id": <card_id>,
// ///             "column_id": <column_id>,
// ///             "description": <card_description>,
// ///             "position": <card_position>
// ///         },
// ///         ...
// ///     ]
// /// }
// /// ```
// #[get("/<board_id>")]
// pub fn boards_get_board(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
//     let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     let board_id = Uuid::try_parse(board_id)
//         .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
//     conn.transaction(|conn| {
//         let _ = board_users_relation::table
//             .filter(
//                 board_users_relation::board_id
//                     .eq(board_id)
//                     .and(board_users_relation::user_id.eq(token)),
//             )
//             .first::<BoardUsersRelation>(conn)?;
//         let board_name = boards::table
//             .filter(boards::id.eq(board_id))
//             .select(boards::name)
//             .first::<String>(conn)?;
//         let columns = board_column::table
//             .filter(board_column::board_id.eq(board_id))
//             .select((board_column::id, board_column::name, board_column::position))
//             .load::<ReturnedColumn>(conn)?
//             .into_iter()
//             .map(|column| PubColumn {
//                 id: column.0,
//                 name: column.1,
//                 position: column.2,
//             })
//             .collect::<Vec<PubColumn>>();
//         let cards = column_card::table
//             .filter(column_card::column_id.eq_any(columns.iter().map(|column| column.id)))
//             .select((
//                 column_card::id,
//                 column_card::name,
//                 column_card::cover_attachment,
//                 column_card::position,
//                 column_card::description,
//                 column_card::column_id,
//             ))
//             .load::<ReturnedCard>(conn)?
//             .into_iter()
//             .map(|card| PubCard {
//                 id: card.0,
//                 name: card.1,
//                 cover_attachment: card.2,
//                 position: card.3,
//                 description: card.4,
//                 column_id: card.5,
//             })
//             .collect::<Vec<PubCard>>();
//         let board = BoardInfo {
//             name: board_name,
//             id: board_id,
//             columns,
//             cards,
//         };
//         Ok::<BoardInfo, diesel::result::Error>(board)
//     })
//     .map(|board| (Json(json!(board))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # PUT /boards/<board_id>
// /// Updates the board with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `board_id` - The id of the board
// #[put("/<board_id>", data = "<board>")]
// pub fn boards_update_board(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
//     board: &str,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
//     let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     let board_id = Uuid::try_parse(board_id)
//         .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
//     conn.transaction(|conn| {
//         let board = diesel::update(
//             boards::table.filter(boards::id.eq(board_id).and(boards::creator_id.eq(token))),
//         )
//         .set(boards::name.eq(board))
//         .returning(boards::id)
//         .get_result::<Uuid>(conn)?;
//         Ok::<Uuid, diesel::result::Error>(board)
//     })
//     .map(|id| (Json(json!(id))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # DELETE /boards/<board_id>
// /// Deletes the board with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `board_id` - The id of the board
// #[delete("/<board_id>")]
// pub fn boards_delete_board(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
//     let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     let board_id = Uuid::try_parse(board_id)
//         .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
//     conn.transaction(|conn| {
//         let column_ids = board_column::table
//             .filter(board_column::board_id.eq(board_id))
//             .select(board_column::id)
//             .load::<Uuid>(conn)?;
//         diesel::delete(column_card::table.filter(column_card::column_id.eq_any(column_ids)))
//             .execute(conn)?;
//         diesel::delete(board_column::table.filter(board_column::board_id.eq(board_id)))
//             .execute(conn)?;
//         diesel::delete(
//             board_users_relation::table.filter(board_users_relation::board_id.eq(board_id)),
//         )
//         .execute(conn)?;
//         let id = diesel::delete(
//             boards::table.filter(boards::id.eq(board_id).and(boards::creator_id.eq(token))),
//         )
//         .returning(boards::id)
//         .get_result::<Uuid>(conn)?;
//         Ok::<Uuid, diesel::result::Error>(id)
//     })
//     .map(|id| (Json(json!(id))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }
