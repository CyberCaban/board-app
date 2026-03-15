use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubAttachment},
};

#[get("/<board_id>/cards/<card_id>/attachments")]
pub async fn boards_get_attachments_of_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    card_id: String,
) -> Result<ApiResponse<Vec<PubAttachment>>, ApiResponse> {
    let auth = auth.unpack()?;
    let attachments = BoardQueries::get_attachments_of_card(&db, auth, board_id, card_id)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(attachments))
}
