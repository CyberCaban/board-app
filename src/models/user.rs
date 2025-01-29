use diesel::{
    query_dsl::methods::FilterDsl, BoolExpressionMethods, ExpressionMethods, Insertable,
    PgConnection, Queryable, QueryableByName, RunQueryDsl, Selectable,
};
use rocket::http::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, RegisterError},
    jwt,
    schema::users,
};

use super::api_response::ApiResponse;
use crate::errors::ApiErrorType::*;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub profile_url: Option<String>,
    pub bio: Option<String>,
    pub friends: Option<Vec<Option<uuid::Uuid>>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
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
#[derive(Serialize, Deserialize)]
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
        let hash = hash(user.password, DEFAULT_COST);
        if let Err(e) = hash {
            return Err(ApiResponse::from_error(ApiError::from_error(e)));
        }
        let user = SignupDTO {
            password: hash.unwrap(),
            ..user
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
                        friends: None,
                        profile_url: None,
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
        user: LoginDTO,
        db: Db,
        jar: &CookieJar<'_>,
    ) -> Result<ApiResponse<PubUser>, ApiResponse<ApiError>> {
        if user.email.is_empty() || user.password.is_empty() {
            return Err(ApiResponse::from_error(ApiError::from_type(EmptyFields)));
        }
        let hash = hash(user.password, DEFAULT_COST);
        if let Err(e) = hash {
            return Err(ApiResponse::from_error(ApiError::from_error(e)));
        }
        let hashed_passwd = hash.unwrap();
        match db
            .run(move |conn| {
                match users::table
                    .filter(users::email.eq(user.email))
                    .filter(users::password.eq(hashed_passwd))
                    .first::<User>(conn)
                {
                    Err(_) => Err(ApiError::from_type(UserNotFound)),
                    Ok(user) => Ok(user),
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
