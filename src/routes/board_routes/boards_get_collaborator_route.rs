use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[get("/<board_id>/collaborators/<collaborator_id>")]
pub async fn boards_get_collaborator(
    db: Db,
    auth: AuthResult,
    board_id: String,
    collaborator_id: String,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let collaborator = BoardQueries::get_collaborator(&db, auth, board_id, collaborator_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(collaborator))
}
