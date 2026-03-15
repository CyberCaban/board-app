use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, NewColumn},
};

#[post("/<board_id>/columns", data = "<column>")]
pub async fn boards_create_column(
    db: Db,
    auth: AuthResult,
    board_id: String,
    column: Json<NewColumn>,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let auth = auth.unpack()?;
    let column_id = BoardQueries::create_column(&db, auth, board_id, column.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(column_id))
}
