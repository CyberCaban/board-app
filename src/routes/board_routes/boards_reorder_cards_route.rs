use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubCard},
};

#[put("/<board_id>/columns/<from_column_id>/cards/<card_id>/reorder/<to_column_id>/<to_pos>")]
pub async fn boards_reorder_cards(
    db: Db,
    auth: AuthResult,
    board_id: String,
    from_column_id: String,
    card_id: String,
    to_column_id: String,
    to_pos: i32,
) -> Result<ApiResponse<Vec<PubCard>>, ApiResponse> {
    let auth = auth.unpack()?;
    let reordered = BoardQueries::reorder_cards(
        &db,
        auth,
        board_id,
        from_column_id,
        card_id,
        to_column_id,
        to_pos,
    )
    .await
    .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(reordered))
}
