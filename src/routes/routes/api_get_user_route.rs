use crate::{
    database::{routes_queries::RoutesQueries, Db},
    models::{api_response::ApiResponse, user::PubUser},
};

#[get("/user/<user_id>")]
pub async fn api_get_user(db: Db, user_id: String) -> Result<ApiResponse<PubUser>, ApiResponse> {
    let user = RoutesQueries::get_user(&db, user_id)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(user.into()))
}
