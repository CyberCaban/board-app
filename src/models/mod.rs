use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, DateTime, Utc};

use crate::schema::column_card;
pub mod api_response;
pub mod auth;

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub profile_url: Option<String>,
    pub bio: Option<String>,
    pub friends: Option<Vec<Option<uuid::Uuid>>>,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::files)]
pub struct UploadedFile {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: uuid::Uuid,
    pub private: bool,
}

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

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::friends_requests)]
pub struct NewFriendRequest {
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
}

#[derive(Queryable, Serialize, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::friends_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FriendsRequests {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub updated_at: NaiveDateTime,
}

#[macro_export]
macro_rules! validate_user_token {
    ($cookies: ident) => {
        match $cookies.get("token") {
            Some(cookie) => match Uuid::parse_str(cookie.value_trimmed()) {
                Ok(upl_id) => upl_id,
                Err(_) => return Err(ApiError::from_type(ApiErrorType::InvalidToken).to_json()),
            },
            None => return Err(ApiError::from_type(ApiErrorType::Unauthorized).to_json()),
        }
    };
    ($cookies: ident, $db: ident) => {
        match $cookies.get("token") {
            None => return Err(ApiError::from_type(ApiErrorType::Unauthorized).to_json()),
            Some(cookie) => match Uuid::parse_str(cookie.value_trimmed()) {
                Err(_) => return Err(ApiError::from_type(ApiErrorType::InvalidToken).to_json()),
                Ok(upl_id) => $db
                    .run(move |conn| {
                        crate::schema::users::table
                            .filter(crate::schema::users::id.eq(upl_id))
                            .first::<crate::models::User>(conn)
                    })
                    .await
                    .map_err(|_| ApiError::from_type(ApiErrorType::InvalidToken).to_json())
                    .map(|usr| usr.id)?,
            },
        }
    };
}
