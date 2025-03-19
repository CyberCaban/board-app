use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::user::{LoginDTO, User},
    schema::users,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    username: String,
    old_password: String,
    new_password: String,
    profile_url: String,
    bio: String,
}
pub struct UserQueries;

impl UserQueries {
    pub async fn find_by_email(db: &Db, email: String) -> Result<User, ApiError> {
        db.run(move |conn| {
            users::table
                .filter(users::email.eq(email))
                .first::<User>(conn)
                .map_err(|_| ApiError::from_type(ApiErrorType::UserNotFound))
        })
        .await
    }

    pub async fn verify_password(db: &Db, login_info: LoginDTO) -> Result<User, ApiError> {
        let user = Self::find_by_email(&db, login_info.email).await?;

        match bcrypt::verify(login_info.password, &user.password) {
            Ok(true) => Ok(user),
            Ok(false) => Err(ApiError::from_type(ApiErrorType::WrongPassword)),
            Err(e) => Err(ApiError::from_error(e)),
        }
    }

    pub async fn create_user(db: &Db, user: User) -> Result<User, diesel::result::Error> {
        db.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&user)
                .get_result::<User>(conn)
        })
        .await
    }

    pub async fn update_profile(
        db: &Db,
        new_user: UpdateUser,
        user_token: Uuid,
    ) -> Result<User, ApiError> {
        db.run(move |conn| {
            diesel::update(users::table)
                .filter(users::id.eq(user_token))
                .set((
                    users::username.eq(&new_user.username.trim()),
                    users::profile_url.eq(&new_user.profile_url.trim()),
                    users::bio.eq(&new_user.bio.trim()),
                ))
                .get_result::<User>(conn)
                .map_err(|e| ApiError::from_error(e))
        })
        .await
    }
}
