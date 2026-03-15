use serde_json::Value;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[get("/<board_id>/columns/<column_id>/cards/<card_id>")]
pub async fn boards_get_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    card_id: String,
) -> Result<ApiResponse<Value>, ApiResponse> {
    let auth = auth.unpack()?;
    let card = BoardQueries::get_card(&db, auth, board_id, column_id, card_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(card))
}
