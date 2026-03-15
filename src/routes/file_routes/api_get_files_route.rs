use rocket::serde::json::json;
use serde_json::Value;

use crate::{
    database::{file_queries::FileQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[get("/files")]
pub async fn api_get_files(db: Db, auth: AuthResult) -> Result<ApiResponse<Value>, ApiResponse> {
    if auth.is_err() {
        return match FileQueries::load_public_files(&db).await {
            Ok(files) => Ok(ApiResponse::new(json!(files))),
            Err(e) => Err(ApiResponse::from_error(e.into())),
        };
    }

    let uploader_id = auth.unpack()?.id;
    match FileQueries::load_private_files(&db, uploader_id).await {
        Ok(files) => Ok(ApiResponse::new(json!(files))),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
