use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[delete("/<board_id>/columns/<column_id>/cards/<card_id>")]
pub async fn boards_delete_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    card_id: String,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let deleted = BoardQueries::delete_card(&db, auth, board_id, column_id, card_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(deleted))
}
