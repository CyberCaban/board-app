use diesel::{Insertable, Queryable, Selectable};
use rocket::{fs::TempFile, tokio::io::AsyncReadExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::files)]
pub struct UploadedFile {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub private: bool,
}

#[derive(FromForm, Debug)]
pub struct UploadRequest<'r> {
    pub file: TempFile<'r>,
    pub filename: String,
    pub is_private: bool,
}

impl UploadedFile {
    pub async fn write_file_to_disk(&self, file: &TempFile<'_>) {
        let mut file = file.open().await.unwrap();
        let mut buf = Vec::new();
        let UploadedFile {
            id,
            name,
            user_id,
            private,
        } = self;
        file.read_to_end(&mut buf).await.unwrap();
        let file_path = if *private {
            format!("tmp/{}/{}", user_id, name)
        } else {
            format!("tmp/{}", name)
        };
        if *private {
            std::fs::create_dir_all(format!("tmp/{}", user_id)).unwrap();
        }
        std::fs::write(file_path, buf).unwrap();
    }
    pub async fn delete_file_from_disk(&self, file_name: String, uploader_id: Uuid) {
        let file_path = if self.private {
            format!("tmp/{}/{}", uploader_id, file_name)
        } else {
            format!("tmp/{}", file_name)
        };
        std::fs::remove_file(file_path).unwrap();
    }
}
