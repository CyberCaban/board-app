use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{form::Form, fs::TempFile, http::CookieJar};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::validate_user_token;
use crate::database::Db;
use crate::errors::{ApiError, ApiErrorType};
use crate::models::{UploadedFile, User};
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
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Json<Value>> {
    if form.file.content_type().is_none() {
        return Err(ApiError::new("InvalidFileType", "Invalid file type").to_json());
    }
    let uploader_id = validate_user_token!(cookies);

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
    let transaction = db
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
        .await;
    match transaction {
        Err(e) => return Err(ApiError::from_error(e).to_json()),
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
            Ok(Json(json!(file_name_clone)))
        }
    }
}

#[get("/file/<file_name>")]
pub async fn api_get_file(
    file_name: String,
    db: Db,
    cookies: &CookieJar<'_>,
) -> Result<NamedFile, Json<Value>> {
    let uploader_id = validate_user_token!(cookies);
    let file_name_clone = file_name.clone();

    let found_file = db
        .run(move |conn| {
            files::table
                .filter(files::name.eq(file_name_clone))
                .filter(files::user_id.eq(uploader_id))
                .first::<UploadedFile>(conn)
        })
        .await;
    match found_file {
        Ok(f) => {
            let file_path = if f.private {
                format!("tmp/{}/{}", uploader_id, file_name)
            } else {
                format!("tmp/{}", file_name)
            };
            NamedFile::open(file_path)
                .await
                .map_err(|e| (ApiError::from_error(e).to_json()))
        }
        Err(_) => Err(ApiError::from_type(ApiErrorType::NotFound).to_json()),
    }
}

#[delete("/file/<file_name>")]
pub async fn api_delete_file(
    file_name: String,
    db: Db,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Json<Value>> {
    let uploader_id = validate_user_token!(cookies);
    let file_name_clone = file_name.clone();
    db.run(move |conn| {
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
    .map(|f| {
        let file_path = if f.private {
            format!("tmp/{}/{}", uploader_id, file_name)
        } else {
            format!("tmp/{}", file_name)
        };
        std::fs::remove_file(file_path).unwrap();
        Json(json!("File deleted"))
    })
    .map_err(|e| ApiError::from_error(e).to_json())
}

#[get("/files")]
pub async fn api_get_files(db: Db, cookies: &CookieJar<'_>) -> Result<Json<Value>, Json<Value>> {
    if cookies.get("token").is_none() {
        return db
            .run(move |conn| {
                files::table
                    .filter(files::private.eq(false))
                    .load::<UploadedFile>(conn)
            })
            .await
            .map(|files| Json(json!(files)))
            .map_err(|e| (ApiError::from_error(e).to_json()));
    }
    let uploader_id = validate_user_token!(cookies);
    db.run(move |conn| {
        files::table
            .filter(files::private.eq(false).or(files::user_id.eq(uploader_id)))
            .load::<UploadedFile>(conn)
    })
    .await
    .map(|files| Json(json!(files)))
    .map_err(|e| (ApiError::from_error(e).to_json()))
}
