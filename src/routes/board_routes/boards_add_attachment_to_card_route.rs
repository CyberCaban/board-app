use rocket::form::Form;
use serde_json::{json, Value};

use crate::{
    database::{board_queries::BoardQueries, Db},
    errors::ApiErrorType,
    models::{api_response::ApiResponse, auth::AuthResult, UploadAttachment},
};

#[post("/<board_id>/cards/<card_id>/attachments", data = "<card>")]
pub async fn boards_add_attachment_to_card(
    db: Db,
    auth: AuthResult,
    board_id: String,
    card_id: String,
    card: Form<UploadAttachment<'_>>,
) -> Result<ApiResponse<Value>, ApiResponse> {
    let auth = auth.unpack()?;
    let (file_name, bytes) = BoardQueries::add_attachment_to_card(&db, auth, board_id, card_id, card.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;

    let file_path = format!("tmp/{}", file_name);
    std::fs::write(&file_path, bytes).map_err(|_| ApiResponse::from_error_type(ApiErrorType::FailedToDeleteFile))?;

    Ok(ApiResponse::new(json!("Attachment added")))
}
