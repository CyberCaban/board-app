use rocket::form::Form;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::database::file_queries::FileQueries;
use crate::database::Db;
use crate::errors::{ApiError, ApiErrorType};
use crate::models::api_response::ApiResponse;
use crate::models::auth::AuthResult;
use crate::models::file::UploadRequest;

// TODO: change path to /file
#[post("/file/create", data = "<form>")]
pub async fn api_upload_file(
    form: Form<UploadRequest<'_>>,
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<String>, ApiResponse> {
    if form.file.content_type().is_none() {
        return Err(ApiResponse::from_error_type(ApiErrorType::InvalidFileType));
    }
    let uploader_id = auth.unpack()?.id;

    match FileQueries::create_file_row(&db, &form, uploader_id).await {
        Err(e) => return Err(ApiResponse::from_error(e.into())),
        Ok(file_record) => {
            file_record.write_file_to_disk(&form.file).await;
            Ok(ApiResponse::new(file_record.name))
        }
    }
}

#[delete("/file/<file_name>")]
pub async fn api_delete_file(
    file_name: String,
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<String>, ApiResponse> {
    let uploader_id = auth.unpack()?.id;
    let file_name_clone = file_name.clone();
    match FileQueries::delete_file_row(&db, file_name_clone, uploader_id).await {
        Ok(f) => {
            f.delete_file_from_disk(file_name, uploader_id).await;
            Ok(ApiResponse::new(
                "Successfully deleted the file".to_string(),
            ))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

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

#[get("/file/<file_id>")]
pub async fn api_get_file(
    db: Db,
    // auth: AuthResult,
    file_id: String,
) -> Result<ApiResponse<Value>, ApiResponse> {
    // let _ = auth.unpack()?;
    let file_id = Uuid::try_parse(&file_id)
        .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
    match FileQueries::load_file_by_id(&db, file_id).await {
        Ok(file) => Ok(ApiResponse::new(json!(file))),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
