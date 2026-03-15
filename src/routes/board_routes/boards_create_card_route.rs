use rocket::serde::json::Json;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, NewCard, PubCard},
};

#[post("/<board_id>/columns/<column_id>/cards", data = "<card>")]
pub async fn boards_create_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    card: Json<NewCard>,
) -> Result<ApiResponse<PubCard>, ApiResponse> {
    let auth = auth.unpack()?;
    let created = BoardQueries::create_card(&db, auth, board_id, column_id, card.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(created))
}
