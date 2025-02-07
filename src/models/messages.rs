use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Default,
    Insertable,
    Queryable,
    QueryableByName,
    Selectable,
)]
#[diesel(table_name = crate::schema::chat_messages)]
pub struct ChatMessageDTO {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub file_id: Option<Uuid>,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ClientMessage> for ChatMessageDTO {
    fn from(value: ClientMessage) -> Self {
        let sender_id = Uuid::parse_str(&value.sender_id).unwrap_or_default();
        let conversation_id = Uuid::parse_str(&value.conversation_id).unwrap_or_default();
        let timestamp = Utc
            .timestamp_millis_opt(value.created_at)
            .unwrap()
            .naive_utc();

        ChatMessageDTO {
            id: Uuid::new_v4(),
            sender_id,
            conversation_id,
            content: value.content,
            file_id: None,
            deleted: false,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ClientMessage {
    pub content: String,
    pub sender_id: String,
    pub conversation_id: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::conversations)]
pub struct Conversation {
    pub id: Uuid,
    pub member_one: Uuid,
    pub member_two: Uuid,
}