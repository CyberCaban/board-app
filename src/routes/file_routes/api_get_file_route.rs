use rocket::serde::json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    database::{file_queries::FileQueries, Db},
    errors::{ApiError, ApiErrorType},
    models::api_response::ApiResponse,
};

#[get("/file/<file_id>")]
pub async fn api_get_file(
    db: Db,
    file_id: String,
) -> Result<ApiResponse<Value>, ApiResponse> {
    let file_id = Uuid::try_parse(&file_id)
        .map_err(|_| ApiResponse::from_error(ApiError::from_type(ApiErrorType::FailedToParseUUID)))?;

    match FileQueries::load_file_by_id(&db, file_id).await {
        Ok(file) => Ok(ApiResponse::new(json!(file))),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
