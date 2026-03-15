use rocket::serde::json::Json;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, NewColumn, PubColumn},
};

#[put("/<board_id>/columns/<column_id>", data = "<column>")]
pub async fn boards_update_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column_id: String,
    column: Json<NewColumn>,
) -> Result<ApiResponse<PubColumn>, ApiResponse> {
    let auth = auth.unpack()?;
    let updated = BoardQueries::update_column(&db, auth, board_id, column_id, column.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(updated))
}
