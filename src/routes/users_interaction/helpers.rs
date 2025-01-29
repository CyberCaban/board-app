use diesel::{
    result::Error, BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use uuid::Uuid;

use crate::{
    models::{FriendList, FriendsRequest, NewFriendRequest},
    schema::{friends_requests, users},
};

pub fn delete_request(conn: &mut PgConnection, token: Uuid, user_id: Uuid) -> Result<usize, Error> {
    diesel::delete(
        friends_requests::table
            .filter(
                friends_requests::sender_id
                    .eq(token)
                    .or(friends_requests::receiver_id.eq(token)),
            )
            .filter(
                friends_requests::sender_id
                    .eq(user_id)
                    .or(friends_requests::receiver_id.eq(user_id)),
            ),
    )
    .execute(conn)
}

pub fn add_request_to_db(conn: &mut PgConnection, token: Uuid, user: Uuid) -> Result<Uuid, Error> {
    diesel::insert_into(friends_requests::table)
        .values(NewFriendRequest {
            sender_id: token,
            receiver_id: user,
        })
        .on_conflict_do_nothing()
        .returning(friends_requests::id)
        .get_result::<Uuid>(conn)
}

pub fn load_user_requests(
    conn: &mut PgConnection,
    token: Uuid,
) -> Result<Vec<FriendsRequest>, Error> {
    friends_requests::table
        .filter(
            friends_requests::receiver_id
                .eq(token)
                .or(friends_requests::sender_id.eq(token)),
        )
        .select(FriendsRequest::as_select())
        .load(conn)
}

pub fn get_user_friends(conn: &mut PgConnection, user_id: Uuid) -> Result<FriendList, Error> {
    users::table
        .filter(users::id.eq(user_id))
        .select(users::friends)
        .first::<FriendList>(conn)
}

pub fn set_user_friends(
    conn: &mut PgConnection,
    user_id: Uuid,
    friends_list: FriendList,
) -> Result<usize, Error> {
    diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::friends.eq(friends_list))
        .execute(conn)
}
