use crate::{
    database::{file_queries::FileQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[delete("/file/<file_name>")]
pub async fn api_delete_file(
    file_name: String,
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<String>, ApiResponse> {
    let uploader_id = auth.unpack()?.id;
    let file_name_clone = file_name.clone();

    match FileQueries::delete_file_row(&db, file_name_clone, uploader_id).await {
        Ok(file) => {
            file.delete_file_from_disk(file_name, uploader_id).await;
            Ok(ApiResponse::new(
                "Successfully deleted the file".to_string(),
            ))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
