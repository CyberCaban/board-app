use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable,
};
use rocket::http::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{database::Db, errors::ApiError, jwt, schema::users};

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
        let users_with_same_email = db
            .run(move |conn| {
                users::table
                    .filter(users::email.eq(email))
                    .count()
                    .get_result::<i64>(conn)
                    .ok()
            })
            .await;
        if let Some(count) = users_with_same_email {
            if count > 0 {
                return Err(ApiResponse::from_error_type(UserAlreadyExists));
            }
        }
        let hash = hash(user.password, DEFAULT_COST);
        if let Err(e) = hash {
            return Err(ApiResponse::from_error(ApiError::from_error(e)));
        }
        let user = SignupDTO {
            password: hash.unwrap(),
            email: user.email,
            username: user.username,
        };
        match db
            .run(move |conn| {
                diesel::insert_into(users::table)
                    .values(&User {
                        id: Uuid::new_v4(),
                        username: user.username,
                        email: user.email,
                        password: user.password,
                        bio: None,
                        profile_url: None,
                        friend_code: None,
                        friend_code_expires_at: None,
                    })
                    .get_result::<User>(conn)
            })
            .await
        {
            Ok(user) => {
                let puser: PubUser = user.into();
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
        match db
            .run(move |conn| {
                match users::table
                    .filter(users::email.eq(login_info.email))
                    .first::<User>(conn)
                {
                    Err(_) => Err::<User, ApiError>(ApiError::from_type(UserNotFound)),
                    Ok(user) => match bcrypt::verify(login_info.password, &user.password) {
                        Err(e) => Err(ApiError::from_error(e)),
                        Ok(res) => {
                            if res {
                                Ok(user)
                            } else {
                                Err(ApiError::from_type(WrongPassword))
                            }
                        }
                    },
                }
            })
            .await
        {
            Ok(user) => {
                let puser: PubUser = user.into();
                jar.add(("token", jwt::Token::generate_token(puser.clone())));
                Ok(ApiResponse::new(puser))
            }
            Err(e) => Err(ApiResponse::from_error(e.into())),
        }
    }
}
