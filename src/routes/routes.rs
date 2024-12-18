use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::time::{Duration, OffsetDateTime};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::database::Db;
use crate::errors::{ApiError, LoginError, RegisterError};
use crate::models::api_response::ApiResponse;
use crate::models::auth::AuthResult;
use crate::models::User;
use crate::schema::users::{self, dsl::*};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    username: String,
    old_password: String,
    new_password: String,
    profile_url: String,
    bio: String,
}

#[get("/user")]
pub async fn api_get_user(db: Db, auth: AuthResult) -> Result<ApiResponse<User>, ApiResponse> {
    let auth = auth.unpack()?;
    match db
        .run(move |conn| {
            users::table
                .filter(users::id.eq(auth.id))
                .first::<User>(conn)
        })
        .await
    {
        Ok(user) => Ok(ApiResponse::new(user)),
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub async fn api_register(
    db: Db,
    user: Json<NewUser>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<User>, ApiResponse> {
    let user = NewUser {
        username: user.username.trim().to_string(),
        password: user.password.trim().to_string(),
    };

    if user.username.is_empty() {
        return Err(ApiResponse::from_error(ApiError::new(
            "EmptyUsername",
            RegisterError::EmptyUsername,
        )));
    }
    if user.password.is_empty() {
        return Err(ApiResponse::from_error(ApiError::new(
            "EmptyPassword",
            RegisterError::EmptyPassword,
        )));
    }
    if user.password.len() < 8 {
        return Err(ApiResponse::from_error(ApiError::new(
            "WeakPassword",
            RegisterError::WeakPassword,
        )));
    }

    match db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&User {
                    id: Uuid::new_v4(),
                    username: user.username.to_string(),
                    password: user.password.to_string(),
                    profile_url: None,
                    bio: None,
                })
                .on_conflict(users::username)
                .do_nothing()
                .get_result::<User>(conn)
        })
        .await
    {
        Ok(user) => {
            let cookie = Cookie::build(("token", user.id.to_string()))
                .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
            jar.add(cookie);
            Ok(ApiResponse::new(user))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[post("/login", format = "json", data = "<user>")]
pub async fn api_login(
    db: Db,
    user: Json<NewUser>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<User>, ApiResponse> {
    let user = user.into_inner();
    if user.username.is_empty() {
        return Err(ApiResponse::from_error(ApiError::new(
            "EmptyUsername",
            LoginError::EmptyUsername,
        )));
    }
    if user.password.is_empty() {
        return Err(ApiResponse::from_error(ApiError::new(
            "EmptyPassword",
            LoginError::EmptyPassword,
        )));
    }

    match db
        .run(move |conn| {
            match users
                .filter(users::username.eq(user.username))
                .first::<User>(conn)
            {
                Err(_) => Err(ApiError::new("UserNotFound", LoginError::UserNotFound)),
                Ok(usr) => {
                    if usr.password != user.password {
                        Err(ApiError::new("WrongPassword", LoginError::WrongPassword))
                    } else {
                        Ok(usr)
                    }
                }
            }
        })
        .await
    {
        Ok(user) => {
            let cookie = Cookie::build(("token", user.id.to_string()))
                .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
            jar.add(cookie);
            Ok(ApiResponse::new(user))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[put("/user", format = "json", data = "<new_user>")]
pub async fn api_update_user(
    db: Db,
    new_user: Json<UpdateUser>,
    jar: &CookieJar<'_>,
    auth: AuthResult,
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let user_token = auth.unpack()?.id;
    let new_user = new_user.into_inner();

    match db
        .run(move |conn| {
            diesel::update(users::table)
                .filter(users::id.eq(user_token))
                .set((
                    users::username.eq(&new_user.username.trim()),
                    users::profile_url.eq(&new_user.profile_url.trim()),
                    users::bio.eq(&new_user.bio.trim()),
                ))
                .returning(users::id)
                .get_result::<Uuid>(conn)
        })
        .await
    {
        Ok(user_id) => {
            let cookie = Cookie::build(("token", user_id.to_string()))
                .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
            jar.add(cookie);
            Ok(ApiResponse::new(user_id))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[post("/logout")]
pub fn api_logout(jar: &CookieJar<'_>) -> Value {
    jar.remove("token");
    json!("Logged out")
}

#[get("/toro", format = "html")]
pub fn toro() -> RawHtml<String> {
    let toro: &str = "
::::::::::::::::::::::..::::::::::::::::::::::::''::''''''''''''''''''''''
::::::::::::::::~~[[%%==..::::''''''''''''::::''''::::::''''''''----~~====
::::::::::::::--[[OO##@@[[''::------------::''--''::--==''::''''----&lt;&lt;&lt;&lt;~~
::::::::::::''**%%88####QQ88''''--------''::''===={{88##88~~''''----**&lt;&lt;&lt;&lt;
::::::::::::~~**[[OO######QQ00::''------''::--((%%008888@@00::''----&lt;&lt;{{**
::::::::::''==**[[88######@@QQ%%~~~~--''''''((%%%%0000OO####&lt;&lt;''------~~--
''''::''::--==**%%88######@@@@@@##OO88008800%%%%008888OOOO##00''--~~======
''....::::--==**%%OO######@@##############@@##OO000088OOOO##OO--~~****((**
::::::::::~~==(([[88############OOOO########@@@@##OO88OOOO##@@&lt;&lt;~~((((((((
::::''''::~~&lt;&lt;{{%%88OOOO######OOOOOO######@@@@@@@@@@@@##OOOO@@{{~~********
::::''''''==&lt;&lt;(([[0088OOOOOOOOOOOOOO##@@####@@@@@@@@@@QQ@@##@@{{~~((**((((
::::''----==&lt;&lt;(([[%%0088OOOOOOOOOOOO####@@@@@@@@@@@@@@QQQQQQQQ{{~~((((((((
::::----~~&lt;&lt;&lt;&lt;(({{%%000088OOOOOOOOOO##@@##@@@@@@@@@@QQQQQQQQ&amp;&amp;**~~**&lt;&lt;**&lt;&lt;
::::::--==&lt;&lt;**{{[[%%8888OOOOOOOOOOOO######@@@@@@@@@@QQQQQQQQ&amp;&amp;{{~~(((({{((
::::::~~&lt;&lt;&lt;&lt;&lt;&lt;&lt;&lt;==~~~~==((%%OO##OOOOOO####@@@@@@@@QQ@@@@@@QQ&amp;&amp;00~~********
::::----''::--''--&lt;&lt;**==~~''~~{{88####@@@@@@QQQQ@@{{==~~==**00@@{{&lt;&lt;******
::::..--**((~~....((8800OO00[[**~~~~&lt;&lt;**[[OO{{**&lt;&lt;~~&lt;&lt;**{{**''{{{{**(((({{
::..''**{{**  ..  ~~%%%%%%%%%%00[[''..  ==%%~~[[00OOOOOO@@{{..(([[--&lt;&lt;==~~
::..--**[[''  ..  &lt;&lt;[[[[[[[[((&lt;&lt;&lt;&lt;--==&lt;&lt;**--((QQ@@########&lt;&lt;..==QQ--''~~::
::..''(([[''    --%%%%%%00%%[[&lt;&lt;~~::{{((&lt;&lt;--&lt;&lt;@@@@@@@@@@QQ{{..''##&lt;&lt;::==''
''::::(([[[[****%%%%%%0000%%{{&lt;&lt;~~''00{{&lt;&lt;--~~QQ@@@@@@@@@@##--::00((--==''
''~~----**{{[[[[000000008800{{&lt;&lt;--==OO[[&lt;&lt;((--[[QQQQ@@@@@@@@88====--''====
''==**==''--~~~~~~&lt;&lt;******&lt;&lt;~~~~&lt;&lt;8888%%((0088==((00OO##@@OO%%(({{~~~~----
--&lt;&lt;********(({{((==&lt;&lt;(((({{%%88##OO8800%%88##@@%%**((((((((**(({{&lt;&lt;''--''
--&lt;&lt;**==&lt;&lt;**{{{{%%{{[[000088OOOOOOOOOOOO8888OO88&lt;&lt;--00OO000088OO##**::~~''
''&lt;&lt;****&lt;&lt;&lt;&lt;**((((((%%0000888888OO######OOOOOO8888==[[OO0088##OO##&lt;&lt;::==''
::&lt;&lt;**(({{(((((((([[888800OO[[**##00((##@@##@@##{{&lt;&lt;OO##OO8888OO##~~::&lt;&lt;--
==~~****(({{{{(({{[[008800OO&lt;&lt;&lt;&lt;[[==**&lt;&lt;%%00%%&lt;&lt;&lt;&lt;OO########@@QQ%%''::==--
&lt;&lt;******(((((((((({{[[00008888****88@@00((**((00##OO####OO##@@@@==''::~~==
&lt;&lt;**((((((((((((****(([[%%00OOOOOO88OO##########OOOO##OOOOOO@@{{::''''--&lt;&lt;
(({{{{((((((((******(((({{008888888888OOOOOOOOOOOOOOOOOOOO##((::''''''''&lt;&lt;
((((((((((((************{{[[0000000000888888OOOO####OOOO%%==::''''''''''&lt;&lt;
**(({{{{((**********((**(({{%%%%%%%%%%00888888[[((((**==''::''''''''''::&lt;&lt;
**(({{((****((((&lt;&lt;(({{**&lt;&lt;(({{{{[[%%88OO##@@@@--..::....::::::''''''''::==
(((({{{{{{{{{{****((((&lt;&lt;==&lt;&lt;(([[88OO@@@@QQQQ&amp;&amp;88%%8800{{&lt;&lt;~~--::::::::..~~
{{(([[{{{{[[{{******&lt;&lt;==&lt;&lt;(({{00OO##@@@@QQQQQQQQQQ@@QQ@@##OO88[[((&lt;&lt;==~~~~
(((([[(({{{{**(({{[[((&lt;&lt;**(({{%%88OO##@@QQQQQQQQQQ@@########OO8888OO888888
    ";
    RawHtml(
        format!(
            "
    <html>
        <head>
            <title>Toro</title>
        </head>
        <body style='background-color: black;'>
            <pre style='font-family: monospace; color: white'>
                {toro}
            </pre>
        </body>
    </html>
    "
        )
        .to_string(),
    )
}
