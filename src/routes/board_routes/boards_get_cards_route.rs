use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubCard},
};

#[get("/<board_id>/columns/<column_id>/cards")]
pub async fn boards_get_cards(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
) -> Result<ApiResponse<Vec<PubCard>>, ApiResponse> {
    let auth = auth.unpack()?;
    let cards = BoardQueries::get_cards(&db, auth, board_id, column_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(cards))
}
