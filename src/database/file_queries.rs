use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::form::Form;
use uuid::Uuid;

use crate::database::Db;
use crate::errors::{ApiError, ApiErrorType};
use crate::models::file::{UploadRequest, UploadedFile};
use crate::models::user::User;
use crate::schema::{files, users};

pub struct FileQueries;

impl FileQueries {
    pub async fn create_file_row(
        db: &Db,
        form: &Form<UploadRequest<'_>>,
        uploader_id: Uuid,
    ) -> Result<UploadedFile, diesel::result::Error> {
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
        db.run(move |conn| {
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
                let file = diesel::insert_into(files::table)
                    .values(&new_file)
                    .get_result::<UploadedFile>(conn)?;
                Ok::<UploadedFile, diesel::result::Error>(file)
            })
        })
        .await
    }

    pub async fn delete_file_row(
        db: &Db,
        file_name: String,
        user_id: Uuid,
    ) -> Result<UploadedFile, diesel::result::Error> {
        db.run(move |conn| {
            conn.transaction(|conn| {
                let f = files::table
                    .filter(files::name.eq(file_name).and(files::user_id.eq(user_id)))
                    .first::<UploadedFile>(conn)
                    .map_err(|_| ApiError::from_type(ApiErrorType::YouDoNotOwnThisFile))?;
                diesel::delete(files::table.filter(files::id.eq(f.id))).execute(conn)?;
                Ok::<UploadedFile, diesel::result::Error>(f)
            })
        })
        .await
    }

    pub async fn load_public_files(db: &Db) -> Result<Vec<UploadedFile>, diesel::result::Error> {
        db.run(move |conn| {
            files::table
                .filter(files::private.eq(false))
                .load::<UploadedFile>(conn)
        })
        .await
    }
    pub async fn load_private_files(
        db: &Db,
        user_id: Uuid,
    ) -> Result<Vec<UploadedFile>, diesel::result::Error> {
        db.run(move |conn| {
            files::table
                .filter(files::private.eq(false).or(files::user_id.eq(user_id)))
                .load::<UploadedFile>(conn)
        })
        .await
    }
}
