use crate::{
    database::{users_interaction_queries::UsersInteractionQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, user::PubUser},
};

#[get("/list")]
pub async fn get_friends(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Vec<PubUser>>, ApiResponse> {
    let auth = auth.unpack()?;
    let friends = UsersInteractionQueries::get_friends(&db, auth)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(friends))
}
