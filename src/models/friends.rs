use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendCode {
    pub code: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendRelationship {
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub created_at: NaiveDateTime,
}
