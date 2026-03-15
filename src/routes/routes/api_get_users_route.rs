use crate::{
    database::{routes_queries::RoutesQueries, Db},
    models::{api_response::ApiResponse, user::PubUser},
};

#[get("/users")]
pub async fn api_get_users(db: Db) -> Result<ApiResponse<Vec<PubUser>>, ApiResponse> {
    let users = RoutesQueries::get_users(&db)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(
        users.into_iter().map(PubUser::from).collect(),
    ))
}
