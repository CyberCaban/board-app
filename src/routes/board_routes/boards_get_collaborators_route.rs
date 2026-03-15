use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[get("/<board_id>/collaborators")]
pub async fn boards_get_collaborators(
    db: Db,
    auth: AuthResult,
    board_id: String,
) -> Result<ApiResponse<Vec<Uuid>>, ApiResponse> {
    let auth = auth.unpack()?;
    let collaborators = BoardQueries::get_collaborators(&db, auth, board_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(collaborators))
}
