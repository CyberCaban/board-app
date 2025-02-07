use crate::database::Db;
use crate::errors::ApiError;
use crate::models::{
    api_response::ApiResponse,
    auth::AuthResult,
    messages::Conversation,
    user::{PubUser, User},
};
use crate::schema::{conversations, users};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use super::helpers::get_conversation_with_members;

#[get("/conversation/<member_one>/<member_two>")]
pub async fn get_conversation(
    db: Db,
    auth: AuthResult,
    member_one: &str,
    member_two: &str,
) -> Result<ApiResponse<(Conversation, PubUser, PubUser)>, ApiResponse<ApiError>> {
    let _ = auth.unpack()?.id;
    let member_one = Uuid::parse_str(member_one).unwrap_or_default();
    let member_two = Uuid::parse_str(member_two).unwrap_or_default();
    get_conversation_with_members(&db, member_one, member_two)
        .await
        .map(|res| ApiResponse::new(res))
        .map_err(|e| ApiResponse::from_error(e.into()))
}

#[post("/conversation/<member_one>/<member_two>")]
pub async fn get_or_create_conversation(
    db: Db,
    auth: AuthResult,
    member_one: &str,
    member_two: &str,
) -> Result<ApiResponse<(Conversation, PubUser, PubUser)>, ApiResponse<ApiError>> {
    let _ = auth.unpack()?.id;
    let member_one = Uuid::parse_str(member_one).map_err(|e| ApiError::from_error(e))?;
    let member_two = Uuid::parse_str(member_two).map_err(|e| ApiError::from_error(e))?;
    let conv = get_conversation_with_members(&db, member_one, member_two).await;

    if let Ok(conv) = conv {
        return Ok(ApiResponse::new(conv));
    }

    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let conversation = diesel::insert_into(conversations::table)
                .values((
                    conversations::member_one.eq(member_one),
                    conversations::member_two.eq(member_two),
                ))
                .get_result::<Conversation>(conn)?;
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
    .map_err(|e| ApiResponse::from_error(e.into()))
    .map(|conversation| ApiResponse::new(conversation))
}
