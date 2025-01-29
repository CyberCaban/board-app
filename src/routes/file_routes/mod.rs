use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::tokio::io::AsyncReadExt;
use rocket::{form::Form, fs::TempFile};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::database::Db;
use crate::errors::{ApiError, ApiErrorType};
use crate::models::api_response::ApiResponse;
use crate::models::auth::AuthResult;
use crate::models::user::User;
use crate::models::UploadedFile;
use crate::schema::{files, users};

#[derive(FromForm, Debug)]
pub struct UploadRequest<'r> {
    pub file: TempFile<'r>,
    pub filename: String,
    pub is_private: bool,
}

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

    let filename = form.filename.clone();
    let file_ext = match form.file.content_type() {
        None => "",
        Some(mime) => {
            let ext = mime.extension();
            match ext {
                None => "",
                Some(ext) => ext.as_str(),
            }
        }
    };
    let is_private = form.is_private;
    let file_name = format!("{}-{}.{}", Uuid::new_v4(), filename, file_ext);
    let file_name_clone = file_name.clone();

    match db
        .run(move |conn| {
            conn.transaction(|conn| {
                let _ = users::table
                    .filter(users::id.eq(uploader_id))
                    .first::<User>(conn)?;
                let new_file = UploadedFile {
                    id: uuid::Uuid::new_v4(),
                    name: file_name,
                    user_id: uploader_id,
                    private: is_private,
                };
                diesel::insert_into(files::table)
                    .values(&new_file)
                    .execute(conn)?;
                Ok::<(), diesel::result::Error>(())
            })
        })
        .await
    {
        Err(e) => return Err(ApiResponse::from_error(e.into())),
        Ok(_) => {
            let mut file = form.file.open().await.unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();
            let file_path = if is_private {
                format!("tmp/{}/{}", uploader_id, file_name_clone)
            } else {
                format!("tmp/{}", file_name_clone)
            };
            if is_private {
                std::fs::create_dir_all(format!("tmp/{}", uploader_id)).unwrap();
            }
            std::fs::write(file_path, buf).unwrap();
            Ok(ApiResponse::new(file_name_clone))
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
    match db
        .run(move |conn| {
            conn.transaction(|conn| {
                let f = files::table
                    .filter(
                        files::name
                            .eq(file_name_clone)
                            .and(files::user_id.eq(uploader_id)),
                    )
                    .first::<UploadedFile>(conn)
                    .map_err(|_| ApiError::from_type(ApiErrorType::YouDoNotOwnThisFile))?;
                diesel::delete(files::table.filter(files::id.eq(f.id))).execute(conn)?;
                Ok::<UploadedFile, diesel::result::Error>(f)
            })
        })
        .await
    {
        Ok(f) => {
            let file_path = if f.private {
                format!("tmp/{}/{}", uploader_id, file_name)
            } else {
                format!("tmp/{}", file_name)
            };
            std::fs::remove_file(file_path).unwrap();
            Ok(ApiResponse::new(file_name))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[get("/files")]
pub async fn api_get_files(db: Db, auth: AuthResult) -> Result<ApiResponse<Value>, ApiResponse> {
    if auth.is_err() {
        return match db
            .run(move |conn| {
                files::table
                    .filter(files::private.eq(false))
                    .load::<UploadedFile>(conn)
            })
            .await
        {
            Ok(files) => Ok(ApiResponse::new(json!(files))),
            Err(e) => Err(ApiResponse::from_error(e.into())),
        };
    }
    let uploader_id = auth.unpack()?.id;
    match db
        .run(move |conn| {
            files::table
                .filter(files::private.eq(false).or(files::user_id.eq(uploader_id)))
                .load::<UploadedFile>(conn)
        })
        .await
    {
        Ok(files) => Ok(ApiResponse::new(json!(files))),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}
