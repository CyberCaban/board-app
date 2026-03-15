use serde_json::{json, Value};

use crate::{
    database::{board_queries::BoardQueries, Db},
    errors::ApiErrorType,
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[delete("/<board_id>/cards/<card_id>/attachments/<attachment_id>")]
pub async fn boards_delete_attachment_of_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    card_id: String,
    attachment_id: String,
) -> Result<ApiResponse<Value>, ApiResponse> {
    let auth = auth.unpack()?;
    let file_name = BoardQueries::delete_attachment_of_card(&db, auth, board_id, card_id, attachment_id)
        .await
        .map_err(ApiResponse::from_error)?;

    std::fs::remove_file(format!("tmp/{}", file_name))
        .map_err(|_| ApiResponse::from_error_type(ApiErrorType::FailedToDeleteFile))?;

    Ok(ApiResponse::new(json!("Attachment deleted")))
}
