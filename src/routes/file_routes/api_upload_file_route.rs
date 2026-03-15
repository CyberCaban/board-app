use rocket::form::Form;

use crate::{
    database::{file_queries::FileQueries, Db},
    errors::ApiErrorType,
    models::{api_response::ApiResponse, auth::AuthResult, file::UploadRequest},
};

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
        Err(e) => Err(ApiResponse::from_error(e.into())),
        Ok(file_record) => {
            file_record.write_file_to_disk(&form.file).await;
            Ok(ApiResponse::new(file_record.name))
        }
    }
}
