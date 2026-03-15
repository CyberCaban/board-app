use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubColumn},
};

#[delete("/<board_id>/columns/<column_id>")]
pub async fn boards_delete_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
) -> Result<ApiResponse<PubColumn>, ApiResponse> {
    let auth = auth.unpack()?;
    let deleted = BoardQueries::delete_column(&db, auth, board_id, column_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(deleted))
}
