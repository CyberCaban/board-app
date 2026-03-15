use rocket::serde::json::Json;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, CardInfo, PubCard},
};

#[put("/<board_id>/columns/<column_id>/cards/<card_id>", data = "<card>")]
pub async fn boards_update_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    card_id: String,
    card: Json<CardInfo>,
) -> Result<ApiResponse<PubCard>, ApiResponse> {
    let auth = auth.unpack()?;
    let updated = BoardQueries::update_card(&db, auth, board_id, column_id, card_id, card.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(updated))
}
