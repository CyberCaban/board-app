use diesel::{
    result::Error, BoolExpressionMethods, Connection, ExpressionMethods, PgConnection, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::schema::friends;

pub fn get_user_friends(conn: &mut PgConnection, user_id: Uuid) -> Result<Vec<Uuid>, Error> {
    friends::table
        .filter(friends::user_id.eq(user_id))
        .select(friends::friend_id)
        .load::<Uuid>(conn)
}