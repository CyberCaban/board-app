use crate::{
    database::{users_interaction_queries::UsersInteractionQueries, Db},
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        messages::Conversation,
        user::PubUser,
    },
};

#[post("/conversation/<member_one>/<member_two>")]
pub async fn get_or_create_conversation(
    db: Db,
    auth: AuthResult,
    member_one: &str,
    member_two: &str,
) -> Result<ApiResponse<(Conversation, PubUser, PubUser)>, ApiResponse> {
    let _ = auth.unpack()?;

    let conversation = UsersInteractionQueries::get_or_create_conversation(
        &db,
        member_one.to_string(),
        member_two.to_string(),
    )
    .await
    .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(conversation))
}
