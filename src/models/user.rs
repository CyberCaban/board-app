use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
};
use rocket::http::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{user_queries::UserQueries, Db},
    errors::ApiError,
    jwt,
    schema::users,
};

use super::api_response::ApiResponse;
use crate::errors::ApiErrorType::*;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile_url: Option<String>,
    pub bio: Option<String>,
    pub friend_code: Option<String>,
    pub friend_code_expires_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(name: String, email: String, passwd: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username: name,
            email,
            password: passwd,
            profile_url: None,
            bio: None,
            friend_code: None,
            friend_code_expires_at: None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct PubUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub profile_url: Option<String>,
    pub bio: Option<String>,
}

impl From<User> for PubUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            profile_url: value.profile_url,
            bio: value.bio,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignupDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}
impl User {
    pub async fn signup(
        user: SignupDTO,
        db: Db,
        jar: &CookieJar<'_>,
    ) -> Result<ApiResponse<PubUser>, ApiResponse<ApiError>> {
        if user.email.is_empty() || user.username.is_empty() || user.password.is_empty() {
            return Err(ApiResponse::from_error(ApiError::from_type(EmptyFields)));
        }
        let email = user.email.clone();
        let users_with_same_email = UserQueries::find_by_email(&db, email).await;
        if let Ok(_) = users_with_same_email {
            return Err(ApiResponse::from_error_type(UserAlreadyExists));
        }
        let hash = hash(user.password, DEFAULT_COST);
        if let Err(e) = hash {
            return Err(ApiResponse::from_error(ApiError::from_error(e)));
        }
        let new_user = User::new(user.username, user.email, hash.unwrap());
        match UserQueries::create_user(&db, new_user).await {
            Ok(new_user) => {
                let puser: PubUser = new_user.into();
                jar.add(("token", jwt::Token::generate_token(puser.clone())));
                Ok(ApiResponse::new(puser))
            }
            Err(e) => Err(ApiResponse::from_error(e.into())),
        }
    }

    pub async fn login(
        login_info: LoginDTO,
        db: Db,
        jar: &CookieJar<'_>,
    ) -> Result<ApiResponse<PubUser>, ApiResponse<ApiError>> {
        if login_info.email.is_empty() || login_info.password.is_empty() {
            return Err(ApiResponse::from_error(ApiError::from_type(EmptyFields)));
        }
        match UserQueries::verify_password(&db, login_info).await {
            Ok(user) => {
                let puser: PubUser = user.into();
                jar.add(("token", jwt::Token::generate_token(puser.clone())));
                Ok(ApiResponse::new(puser))
            }
            Err(e) => Err(ApiResponse::from_error(e.into())),
        }
    }
}
