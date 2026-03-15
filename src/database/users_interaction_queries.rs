use chrono::Utc;
use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        auth::Auth,
        messages::{ChatMessageDTO, ClientMessage, Conversation},
        user::{PubUser, User},
    },
    schema::{chat_messages, conversations, friends, users},
};

pub struct UsersInteractionQueries;

pub struct SendMessageResult {
    pub saved_message: ChatMessageDTO,
    pub sender_username: String,
    pub recipient_id: Uuid,
}

impl UsersInteractionQueries {
    pub async fn get_friends(db: &Db, auth: Auth) -> Result<Vec<PubUser>, ApiError> {
        db.run(move |conn| {
            let ids = friends::table
                .filter(friends::user_id.eq(auth.id))
                .select(friends::friend_id)
                .load::<Uuid>(conn)
                .map_err(ApiError::from)?;

            let users_data = users::table
                .filter(users::id.eq_any(ids))
                .select(users::all_columns)
                .load::<User>(conn)
                .map_err(ApiError::from)?;

            Ok::<Vec<PubUser>, ApiError>(users_data.into_iter().map(PubUser::from).collect())
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_last_messages(
        db: &Db,
        conversation_id: String,
    ) -> Result<Vec<ChatMessageDTO>, ApiError> {
        let conversation_id = parse_uuid(&conversation_id)?;

        db.run(move |conn| {
            conn.transaction::<_, ApiError, _>(|conn| {
                chat_messages::table
                    .filter(chat_messages::conversation_id.eq(conversation_id))
                    .order_by(chat_messages::created_at.desc())
                    .limit(100)
                    .load::<ChatMessageDTO>(conn)
                    .map_err(ApiError::from)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_conversation_with_members(
        db: &Db,
        member_one: String,
        member_two: String,
    ) -> Result<(Conversation, PubUser, PubUser), ApiError> {
        let member_one = parse_uuid(&member_one)?;
        let member_two = parse_uuid(&member_two)?;

        db.run(move |conn| {
            conn.transaction::<_, ApiError, _>(|conn| {
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
                    .first::<Conversation>(conn)
                    .map_err(ApiError::from)?;

                let first: PubUser = users::table
                    .filter(users::id.eq(member_one))
                    .first::<User>(conn)
                    .map_err(ApiError::from)?
                    .into();
                let second: PubUser = users::table
                    .filter(users::id.eq(member_two))
                    .first::<User>(conn)
                    .map_err(ApiError::from)?
                    .into();

                Ok::<(Conversation, PubUser, PubUser), ApiError>((conversation, first, second))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_or_create_conversation(
        db: &Db,
        member_one: String,
        member_two: String,
    ) -> Result<(Conversation, PubUser, PubUser), ApiError> {
        let member_one_id = parse_uuid(&member_one)?;
        let member_two_id = parse_uuid(&member_two)?;

        if let Ok(existing) =
            Self::get_conversation_with_members(db, member_one.clone(), member_two.clone()).await
        {
            return Ok(existing);
        }

        db.run(move |conn| {
            conn.transaction::<_, ApiError, _>(|conn| {
                let conversation = diesel::insert_into(conversations::table)
                    .values((
                        conversations::member_one.eq(member_one_id),
                        conversations::member_two.eq(member_two_id),
                    ))
                    .get_result::<Conversation>(conn)
                    .map_err(ApiError::from)?;

                let first: PubUser = users::table
                    .filter(users::id.eq(member_one_id))
                    .first::<User>(conn)
                    .map_err(ApiError::from)?
                    .into();
                let second: PubUser = users::table
                    .filter(users::id.eq(member_two_id))
                    .first::<User>(conn)
                    .map_err(ApiError::from)?
                    .into();

                Ok::<(Conversation, PubUser, PubUser), ApiError>((conversation, first, second))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn send_message(
        db: &Db,
        auth: Auth,
        message: ClientMessage,
    ) -> Result<SendMessageResult, ApiError> {
        let conversation_id = parse_uuid(&message.conversation_id)?;
        let content = message.content.trim().to_string();
        if content.is_empty() {
            return Err(ApiError::from_type(ApiErrorType::EmptyFields));
        }

        let sender_id = auth.id;
        let file_id = message.file_id;

        let (conversation, sender_username) = db
            .run(move |conn: &mut diesel::PgConnection| {
                let conv = conversations::table
                    .filter(conversations::id.eq(conversation_id))
                    .first::<Conversation>(conn)
                    .map_err(ApiError::from)?;

                let sender = users::table
                    .filter(users::id.eq(sender_id))
                    .first::<User>(conn)
                    .map_err(ApiError::from)?;

                Ok::<(Conversation, String), ApiError>((conv, sender.username))
            })
            .await
            .map_err(|e| e)?;

        if sender_id != conversation.member_one && sender_id != conversation.member_two {
            return Err(ApiError::from_type(ApiErrorType::Unauthorized));
        }

        let recipient_id = if sender_id == conversation.member_one {
            conversation.member_two
        } else {
            conversation.member_one
        };

        let timestamp = Utc::now().naive_utc();
        let new_message = ChatMessageDTO {
            id: Uuid::new_v4(),
            sender_id,
            conversation_id,
            content,
            file_id,
            deleted: false,
            created_at: timestamp,
            updated_at: timestamp,
        };

        let saved_message = db
            .run(move |conn| {
                diesel::insert_into(chat_messages::table)
                    .values::<ChatMessageDTO>(new_message)
                    .get_result::<ChatMessageDTO>(conn)
                    .map_err(ApiError::from)
            })
            .await
            .map_err(|e| e)?;

        Ok(SendMessageResult {
            saved_message,
            sender_username,
            recipient_id,
        })
    }
}

fn parse_uuid(value: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(value).map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))
}
