use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, BoardInfo},
};

#[get("/<board_id>")]
pub async fn boards_get_board(
    db: Db,
    auth: AuthResult,
    board_id: &str,
) -> Result<ApiResponse<BoardInfo>, ApiResponse> {
    let auth = auth.unpack()?;
    let board = BoardQueries::get_board(&db, auth, board_id.to_string())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(board))
}
