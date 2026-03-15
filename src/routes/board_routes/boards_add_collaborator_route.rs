use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[post("/<board_id>/collaborators", data = "<collaborator_id>")]
pub async fn boards_add_collaborator(
    db: Db,
    auth: AuthResult,
    board_id: String,
    collaborator_id: Json<Uuid>,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let id = BoardQueries::add_collaborator(&db, auth, board_id, collaborator_id.0)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(id))
}
