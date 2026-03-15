use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[put("/<board_id>", data = "<board>")]
pub async fn boards_update_board(
    db: Db,
    auth: AuthResult,
    board_id: &str,
    board: String,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let updated = BoardQueries::update_board(&db, auth, board_id.to_string(), board)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(updated))
}
