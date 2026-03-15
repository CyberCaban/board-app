use crate::{
    database::{board_queries::BoardQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, PubBoard},
};

#[get("/")]
pub async fn boards_get_boards(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Vec<PubBoard>>, ApiResponse> {
    let auth = auth.unpack()?;
    let boards = BoardQueries::get_boards(&db, auth)
        .await
        .map_err(ApiResponse::from_error)?;
    Ok(ApiResponse::new(boards))
}
