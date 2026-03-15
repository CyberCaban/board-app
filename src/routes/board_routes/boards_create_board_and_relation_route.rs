use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, NewBoard},
};

#[post("/", data = "<board>")]
pub async fn boards_create_board_and_relation(
    db: Db,
    auth: AuthResult,
    board: Json<NewBoard>,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let board_id = BoardQueries::create_board_and_relation(&db, auth, board.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(board_id))
}
