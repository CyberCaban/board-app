use chrono::Utc;
use diesel::{
    query_dsl::methods::FilterDsl, BoolExpressionMethods, ExpressionMethods, RunQueryDsl,
};
use jwt::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{database::Db, models::user::{PubUser, User}, schema::users};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
#[derive(Serialize, Deserialize)]
pub struct Token {
    iat: i64,
    exp: i64,
    pub user: PubUser,
}
impl Token {
    pub fn generate_token(user: PubUser) -> String {
        let now = Utc::now().timestamp_millis() / 1_000; // in seconds
        let payload = Token {
            iat: now,
            exp: now + ONE_WEEK,
            user,
        };
        jwt::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(include_bytes!("secret.key")),
        )
        .unwrap()
    }
    pub fn decode_token(token: String) -> Result<jwt::TokenData<Token>, jwt::errors::Error> {
        jwt::decode(
            token.as_str(),
            &DecodingKey::from_secret(include_bytes!("secret.key")),
            &Validation::default(),
        )
    }
    pub async fn verify_token(token: String, db: Db) -> bool {
        let decoded = Token::decode_token(token);
        if let Err(_) = decoded {
            return false;
        }
        let user = decoded.unwrap().claims.user;
        db.run(move |conn| {
            users::table
                .filter(users::id.eq(user.id).and(users::username.eq(user.username)))
                .first::<User>(conn)
                .is_ok()
        })
        .await
    }
}
