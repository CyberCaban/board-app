use crate::{
    database::{routes_queries::RoutesQueries, Db},
    models::{api_response::ApiResponse, auth::AuthResult, user::PubUser},
};

#[get("/user")]
pub async fn api_get_self(db: Db, auth: AuthResult) -> Result<ApiResponse<PubUser>, ApiResponse> {
    let auth = auth.unpack()?;
    let user = RoutesQueries::get_self(&db, auth.id)
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ApiResponse::new(user.into()))
}
