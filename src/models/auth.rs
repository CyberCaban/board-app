use std::convert::Infallible;

use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::request::*;
use uuid::Uuid;

use crate::errors::ApiErrorType;
use crate::{database::Db, schema::users};

use super::api_response::ApiResponse;
use super::User;

#[derive(Debug, PartialEq)]
pub enum AuthResult {
    Success(Auth),
    Failure(ApiErrorType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Auth {
    pub id: uuid::Uuid,
}

impl AuthResult {
    pub fn is_err(&self) -> bool {
        match self {
            AuthResult::Success(_) => false,
            AuthResult::Failure(_) => true,
        }
    }
    pub fn is_ok(&self) -> bool {
        !self.is_err()
    }
    pub fn unpack(&self) -> Result<Auth, ApiResponse> {
        match self {
            AuthResult::Success(auth) => Ok(*auth),
            AuthResult::Failure(error) => Err(ApiResponse::from_error_type(error.clone())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthResult {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Success(AuthResult::Failure(ApiErrorType::Unauthorized)),
            Some(auth_header) => {
                let token = if let Some(token) = auth_header.strip_prefix("Bearer ") {
                    token
                } else {
                    return Outcome::Success(AuthResult::Failure(ApiErrorType::InvalidToken));
                };
                let token = match Uuid::try_parse(token) {
                    Ok(token) => token,
                    Err(_) => {
                        return Outcome::Success(AuthResult::Failure(ApiErrorType::InvalidToken));
                    }
                };

                let db = req.guard::<Db>().await.unwrap();
                let user = db
                    .run(move |conn| users::table.filter(users::id.eq(token)).first::<User>(conn))
                    .await;
                match user {
                    Ok(user) => Outcome::Success(AuthResult::Success(Auth { id: user.id })),
                    _ => Outcome::Success(AuthResult::Failure(ApiErrorType::InvalidToken)),
                }
            }
        }
    }
}
