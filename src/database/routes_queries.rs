use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::user::User,
    schema::users,
};

pub struct RoutesQueries;

impl RoutesQueries {
    pub async fn get_self(db: &Db, user_id: Uuid) -> Result<User, ApiError> {
        db.run(move |conn| {
            users::table
                .filter(users::id.eq(user_id))
                .first::<User>(conn)
        })
        .await
        .map_err(ApiError::from)
    }

    pub async fn get_users(db: &Db) -> Result<Vec<User>, ApiError> {
        db.run(move |conn| users::table.select(users::all_columns).load::<User>(conn))
            .await
            .map_err(ApiError::from)
    }

    pub async fn get_user(db: &Db, user_id: String) -> Result<User, ApiError> {
        let parsed = Uuid::parse_str(&user_id)
            .map_err(|_| ApiError::from_type(ApiErrorType::InvalidUserId))?;

        db.run(move |conn| {
            users::table
                .filter(users::id.eq(parsed))
                .first::<User>(conn)
        })
        .await
        .map_err(ApiError::from)
    }
}
