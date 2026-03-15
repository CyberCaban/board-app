use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[delete("/<board_id>")]
pub async fn boards_delete_board(
    db: Db,
    auth: AuthResult,
    board_id: &str,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let deleted = BoardQueries::delete_board(&db, auth, board_id.to_string())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(deleted))
}
