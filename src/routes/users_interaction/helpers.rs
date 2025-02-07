use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::database::Db;
use crate::errors::ApiError;
use crate::models::{
    api_response::ApiResponse,
    messages::{ChatMessageDTO, Conversation},
    user::{PubUser, User},
};
use crate::schema::{chat_messages, conversations, users};

pub async fn get_last_messages(
    db: Db,
    conversation_id: Uuid,
) -> Result<ApiResponse<Vec<ChatMessageDTO>>, ApiResponse<ApiError>> {
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let messages = chat_messages::table
                .filter(chat_messages::conversation_id.eq(conversation_id))
                .order_by(chat_messages::created_at.desc())
                .limit(100)
                .load::<ChatMessageDTO>(conn)?;
            Ok(messages)
        })
    })
    .await
    .map_err(|e| ApiResponse::from_error(e.into()))
    .map(|messages| ApiResponse::new(messages))
}

pub async fn get_conversation_with_members(
    db: &Db,
    member_one: Uuid,
    member_two: Uuid,
) -> Result<(Conversation, PubUser, PubUser), ApiError> {
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let conversation = conversations::table
                .filter(
                    conversations::member_one
                        .eq(member_one)
                        .or(conversations::member_two.eq(member_one)),
                )
                .filter(
                    conversations::member_two
                        .eq(member_two)
                        .or(conversations::member_one.eq(member_two)),
                )
                .first::<Conversation>(conn)?;
            let member_one: PubUser = users::table
                .filter(users::id.eq(member_one))
                .first::<User>(conn)?
                .into();
            let member_two: PubUser = users::table
                .filter(users::id.eq(member_two))
                .first::<User>(conn)?
                .into();
            Ok((conversation, member_one, member_two))
        })
    })
    .await
    .map_err(|e| (e.into()))
}
