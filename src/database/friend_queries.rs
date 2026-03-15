use chrono::{Local, NaiveDateTime};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::friends::FriendRelationship,
    schema::{friends, users},
};

pub struct FriendQueries;

impl FriendQueries {
    pub async fn set_friend_code(
        db: &Db,
        user_id: Uuid,
        code: String,
        expires_at: NaiveDateTime,
    ) -> Result<(), ApiError> {
        db.run(move |conn| {
            diesel::update(users::table.find(user_id))
                .set((
                    users::friend_code.eq(code),
                    users::friend_code_expires_at.eq(expires_at),
                ))
                .execute(conn)
        })
        .await
        .map(|_| ())
        .map_err(map_diesel_error)
    }

    pub async fn get_friend_code(
        db: &Db,
        user_id: Uuid,
    ) -> Result<(Option<String>, Option<NaiveDateTime>), ApiError> {
        db.run(move |conn| {
            users::table
                .find(user_id)
                .select((users::friend_code, users::friend_code_expires_at))
                .first::<(Option<String>, Option<NaiveDateTime>)>(conn)
        })
        .await
        .map_err(map_diesel_error)
    }

    pub async fn friend_code_exists(db: &Db, code: String) -> Result<bool, ApiError> {
        db.run(move |conn| {
            users::table
                .filter(users::friend_code.eq(code))
                .count()
                .get_result::<i64>(conn)
        })
        .await
        .map(|count| count > 0)
        .map_err(map_diesel_error)
    }

    pub async fn redeem_friend_code(
        db: &Db,
        user_id: Uuid,
        code: String,
    ) -> Result<FriendRelationship, ApiError> {
        db.run(move |conn| {
            conn.transaction(|conn| {
                let friend = users::table
                    .filter(users::friend_code.eq(&code))
                    .filter(users::friend_code_expires_at.gt(Local::now().naive_utc()))
                    .select((users::id, users::friend_code, users::friend_code_expires_at))
                    .first::<(Uuid, Option<String>, Option<NaiveDateTime>)>(conn)
                    .map_err(map_diesel_error)?;

                if friend.0 == user_id {
                    return Err(ApiError::from_type(ApiErrorType::InvalidRequest));
                }

                diesel::insert_into(friends::table)
                    .values(&vec![
                        (
                            friends::user_id.eq(user_id),
                            friends::friend_id.eq(friend.0),
                        ),
                        (
                            friends::user_id.eq(friend.0),
                            friends::friend_id.eq(user_id),
                        ),
                    ])
                    .execute(conn)
                    .map_err(map_diesel_error)?;

                diesel::update(users::table.find(friend.0))
                    .set((
                        users::friend_code.eq::<Option<String>>(None),
                        users::friend_code_expires_at.eq::<Option<NaiveDateTime>>(None),
                    ))
                    .execute(conn)
                    .map_err(map_diesel_error)?;

                Ok(FriendRelationship {
                    user_id,
                    friend_id: friend.0,
                    created_at: Local::now().naive_utc(),
                })
            })
        })
        .await
        .map_err(|e| e)
    }
}

fn map_diesel_error(error: DieselError) -> ApiError {
    match error {
        DieselError::NotFound => ApiError::from_type(ApiErrorType::NotFound),
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            ApiError::from_type(ApiErrorType::AlreadyFriends)
        }
        _ => ApiError::from_error(error),
    }
}
