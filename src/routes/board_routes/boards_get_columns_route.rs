use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubColumn},
};

#[get("/<board_id>/columns")]
pub async fn boards_get_columns(
    db: Db,
    auth: AuthResult,
    board_id: String,
) -> Result<ApiResponse<Vec<PubColumn>>, ApiResponse> {
    let auth = auth.unpack()?;
    let columns = BoardQueries::get_columns(&db, auth, board_id)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(columns))
}
