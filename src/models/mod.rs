use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::column_card;
pub mod api_response;
pub mod auth;
pub mod file;
pub mod friends;
pub mod messages;
pub mod user;
pub mod ws_state;

#[derive(Serialize, Deserialize)]
pub struct PubBoard {
    pub id: uuid::Uuid,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct NewBoard {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct BoardInfo {
    pub id: uuid::Uuid,
    pub name: String,
    pub columns: Vec<PubColumn>,
    pub cards: Vec<PubCard>,
}
#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::boards)]
pub struct Board {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub creator_id: uuid::Uuid,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::board_users_relation)]
pub struct BoardUsersRelation {
    pub board_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct PubColumn {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub position: i32,
}
#[derive(Serialize, Deserialize)]
pub struct NewColumn {
    pub name: Option<String>,
    pub position: i32,
}
pub type ReturnedColumn = (uuid::Uuid, Option<String>, i32);
#[derive(QueryableByName, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::board_column)]
pub struct BoardColumn {
    pub id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub position: i32,
    pub board_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct PubCard {
    pub id: uuid::Uuid,
    pub name: String,
    pub cover_attachment: Option<String>,
    pub position: i32,
    pub description: Option<String>,
    pub column_id: uuid::Uuid,
}
#[derive(Serialize, Deserialize)]
pub struct NewCard {
    pub name: String,
    pub position: i32,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct CardInfo {
    pub name: String,
    pub description: String,
}
pub const SELECT_CARD: (
    column_card::id,
    column_card::name,
    column_card::cover_attachment,
    column_card::position,
    column_card::description,
    column_card::column_id,
) = (
    column_card::id,
    column_card::name,
    column_card::cover_attachment,
    column_card::position,
    column_card::description,
    column_card::column_id,
);
pub type ReturnedCard = (Uuid, String, Option<String>, i32, Option<String>, Uuid);
#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::column_card)]
pub struct ColumnCard {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub column_id: uuid::Uuid,
    pub position: i32,
    pub description: Option<String>,
}

#[derive(FromForm)]
pub struct UploadAttachment<'f> {
    pub file: TempFile<'f>,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct PubAttachment {
    pub id: uuid::Uuid,
    pub url: String,
}
