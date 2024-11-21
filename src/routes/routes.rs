use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::time::{Duration, OffsetDateTime};
use rocket::{serde::json::Json, State};
use serde_json::{json, Value};

use crate::database::PSQLConnection;
use crate::errors::{ApiError, LoginError, RegisterError};
use crate::models;
use crate::models::User;
use crate::schema::users::{self, dsl::*};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateUser<'a> {
    username: &'a str,
    old_password: &'a str,
    new_password: &'a str,
    profile_url: &'a str,
    bio: &'a str,
}

#[get("/user")]
pub fn api_get_user(
    db: &State<PSQLConnection>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Json<Value>> {
    use self::models::User;
    use uuid::Uuid;
    let user_token = cookies.get("token");
    let user_token = match user_token {
        Some(cookie) => match Uuid::parse_str(cookie.value_trimmed()) {
            Ok(upl_id) => upl_id,
            Err(_) => return Err(ApiError::new("InvalidToken", "Invalid token").to_json()),
        },
        None => return Err(ApiError::new("Unauthorized", "Unauthorized").to_json()),
    };
    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };
    match users
        .filter(users::id.eq(user_token))
        .first::<User>(&mut *conn)
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(ApiError::from_error(&e).to_json()),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn api_register(
    db: &State<PSQLConnection>,
    user: Json<NewUser<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Json<Value>> {
    use self::models::User;
    use crate::schema;

    let user = NewUser {
        username: &user.username.trim(),
        password: &user.password.trim(),
    };

    if user.username.is_empty() {
        return Err(ApiError::new("EmptyUsername", RegisterError::EmptyUsername).to_json());
    }

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };
    match users
        .filter(users::username.eq(user.username))
        .first::<User>(&mut *conn)
    {
        Ok(_) => {
            return Err(
                ApiError::new("UserAlreadyExists", RegisterError::UserAlreadyExists).to_json(),
            )
        }
        Err(_) => {
            if user.password.len() < 8 {
                return Err(ApiError::new("WeakPassword", RegisterError::WeakPassword).to_json());
            }
            let new_user = User {
                id: uuid::Uuid::new_v4(),
                username: user.username.to_string(),
                password: user.password.to_string(),
                profile_url: None,
                bio: None,
            };
            if let Err(e) = diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(&mut *conn)
            {
                return Err(ApiError::from_error(&e).to_json());
            }

            cookies.add(("token", new_user.id.to_string()));
            Ok(Json(new_user))
        }
    }
}

#[post("/login", format = "json", data = "<user>")]
pub fn api_login(
    db: &State<PSQLConnection>,
    user: Json<NewUser<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Json<Value>> {
    use self::models::User;

    let user = NewUser {
        username: &user.username.trim(),
        password: &user.password.trim(),
    };

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };
    let usrs = users
        .filter(users::username.eq(user.username))
        .first::<User>(&mut *conn);
    match usrs {
        Err(_) => Err(ApiError::new("UserNotFound", LoginError::UserNotFound).to_json()),
        Ok(usr) => {
            if usr.password != user.password {
                Err(ApiError::new("WrongPassword", LoginError::WrongPassword).to_json())
            } else {
                let cookie = Cookie::build(("token", usr.id.to_string()))
                    .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
                cookies.add(cookie);
                Ok(Json(usr))
            }
        }
    }
}

#[put("/user", format = "json", data = "<new_user>")]
pub fn api_update_user(
    db: &State<PSQLConnection>,
    new_user: Json<UpdateUser<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Json<Value>> {
    use self::models::User;
    use uuid::Uuid;

    let user_token = cookies.get("token");
    let user_token = match user_token {
        Some(cookie) => match Uuid::parse_str(cookie.value_trimmed()) {
            Ok(upl_id) => upl_id,
            Err(_) => return Err(ApiError::new("InvalidToken", "Invalid token").to_json()),
        },
        None => return Err(ApiError::new("Unauthorized", "Unauthorized").to_json()),
    };

    let mut conn = match db.get() {
        Ok(c) => c,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };

    let found_user = match users
        .filter(users::id.eq(user_token))
        .first::<User>(&mut *conn)
    {
        Ok(usr) => usr,
        Err(e) => return Err(ApiError::from_error(&e).to_json()),
    };

    // if found_user.password != new_user.old_password {
    //     return Err(ApiError::new("WrongPassword", LoginError::WrongPassword).to_json());
    // }

    // let new_user = User {
    //     id: found_user.id,
    //     username: (&new_user.username.trim()).to_string(),
    //     password: (&new_user.new_password.trim()).to_string(),
    //     profile_url: Some((&new_user.profile_url.trim()).to_string()),
    //     bio: Some((&new_user.bio.trim()).to_string()),
    // };

    if let Err(e) = diesel::update(users::table)
        .filter(users::id.eq(user_token))
        // .values(&new_user)
        // .on_conflict(users::id)
        // .do_update()
        .set((
            users::username.eq(&new_user.username.trim()),
            // users::password.eq(&new_user.password),
            users::profile_url.eq(&new_user.profile_url.trim()),
            users::bio.eq(&new_user.bio.trim()),
        ))
        .execute(&mut *conn)
    {
        return Err(ApiError::from_error(&e).to_json());
    } else {
        Ok(Json(json!(new_user.into_inner())))
    }
}

#[post("/logout")]
pub fn api_logout(cookies: &CookieJar<'_>) -> Value {
    cookies.remove("token");
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
