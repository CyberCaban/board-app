use crate::{
    database::{users_interaction_queries::UsersInteractionQueries, Db},
    models::{api_response::ApiResponse, messages::ChatMessageDTO},
};

#[get("/last_messages/<conversation_id>")]
pub async fn last_messages(
    db: Db,
    conversation_id: String,
) -> Result<ApiResponse<Vec<ChatMessageDTO>>, ApiResponse> {
    let messages = UsersInteractionQueries::get_last_messages(&db, conversation_id)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(messages))
}
