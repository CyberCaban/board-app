// use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
// use rocket::{http::CookieJar, serde::json::Json, State};
// use serde_json::{json, Value};
// use uuid::Uuid;

// use crate::{
//     check_user_token, connect_db,
//     database::PgConnection,
//     errors::{ApiError, ApiErrorType},
//     models::
//         BoardUsersRelation
//     ,
//     schema::{board_users_relation, boards},
// };

// /// # POST /boards/<board_id>/collaborators
// /// Adds a collaborator to the board with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `collaborator_id` - The id of the collaborator
// #[post("/<board_id>/collaborators", data = "<collaborator_id>")]
// pub fn boards_add_collaborator(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
//     collaborator_id: Json<Uuid>,
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
//         let collaborator = diesel::insert_into(board_users_relation::table)
//             .values(BoardUsersRelation {
//                 board_id: board_id,
//                 user_id: collaborator_id.0,
//             })
//             .returning(board_users_relation::user_id)
//             .get_result::<Uuid>(conn)?;
//         Ok::<Uuid, diesel::result::Error>(collaborator)
//     })
//     .map(|collaborator| (Json(json!(collaborator))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # GET /boards/<board_id>/collaborators
// /// Returns all the collaborators of the board with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `collaborators` - A list of collaborators id's of the board
// #[get("/<board_id>/collaborators")]
// pub fn boards_get_collaborators(
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
//         let ids = board_users_relation::table
//             .filter(board_users_relation::board_id.eq(board_id))
//             .select(board_users_relation::user_id)
//             .load::<Uuid>(conn)?;
//         Ok::<Vec<Uuid>, diesel::result::Error>(ids)
//     })
//     .map(|collaborators| (Json(json!(collaborators))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # GET /boards/<board_id>/collaborators/<collaborator_id>
// /// Returns the collaborator with the given id
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `collaborator_id` - The id of the collaborator
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `collaborator_id` - The id of the collaborator
// #[get("/<board_id>/collaborators/<collaborator_id>")]
// pub fn boards_get_collaborator(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
//     collaborator_id: &str,
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
//         let collaborator =
//             board_users_relation::table
//                 .filter(board_users_relation::board_id.eq(board_id).and(
//                     board_users_relation::user_id.eq(Uuid::parse_str(collaborator_id).unwrap()),
//                 ))
//                 .select(board_users_relation::user_id)
//                 .first::<Uuid>(conn)?;
//         Ok::<Uuid, diesel::result::Error>(collaborator)
//     })
//     .map(|collaborator| (Json(json!(collaborator))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }

// /// # DELETE /boards/<board_id>/collaborators/<collaborator_id>
// /// Removes the collaborator with the given id from the board
// /// # Arguments
// /// * `board_id` - The id of the board
// /// * `collaborator_id` - The id of the collaborator
// /// * `cookies` - Takes the token of the user
// /// # Returns
// /// * `collaborator_id` - The id of the collaborator
// #[delete("/<board_id>/collaborators/<collaborator_id>")]
// pub fn boards_remove_collaborator(
//     db: &State<PgConnection>,
//     cookies: &CookieJar<'_>,
//     board_id: &str,
//     collaborator_id: &str,
// ) -> Result<Json<Value>, Json<Value>> {
//     let mut conn = connect_db!(db);
//     let token = check_user_token!(cookies, conn);
//     let conn = &mut *conn;
//     let board_id = Uuid::try_parse(board_id)
//         .map_err(|_| return ApiError::from_type(ApiErrorType::FailedToParseUUID).to_json())?;
//     conn.transaction(|conn| {
//         // Check if the user is the member of the board
//         let _ = board_users_relation::table
//             .filter(
//                 board_users_relation::board_id
//                     .eq(board_id)
//                     .and(board_users_relation::user_id.eq(token)),
//             )
//             .first::<BoardUsersRelation>(conn)?;
//         // Check if the user is the creator of the board
//         let _ = boards::table
//             .filter(boards::id.eq(board_id).and(boards::creator_id.eq(token)))
//             .select(boards::id)
//             .first::<Uuid>(conn)?;
//         let collaborator_id = Uuid::parse_str(collaborator_id).unwrap();
//         // Delete the collaborator
//         diesel::delete(board_users_relation::table)
//             .filter(
//                 board_users_relation::board_id
//                     .eq(board_id)
//                     .and(board_users_relation::user_id.eq(collaborator_id)),
//             )
//             .execute(conn)?;
//         Ok::<Uuid, diesel::result::Error>(collaborator_id)
//     })
//     .map(|collaborator_id| (Json(json!(collaborator_id))))
//     .map_err(|e| (ApiError::from_error(&e).to_json()))
// }
