use actix_web::{
    cookie::{time::Duration, Cookie, CookieBuilder},
    get, post,
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder, Scope,
};
use fancy_regex::Regex;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::{auth::auth::create_jwt_for_cookie, auth::auth::UserAuth, Random, Secret};

pub fn get_user_scope() -> Scope {
    let pw_reg = Regex::new("^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9]).{8,}$").unwrap();

    Scope::new("")
        .app_data(Data::new(pw_reg))
        .service(register_page)
        .service(login_page)
        .service(login)
        .service(register)
        .service(logout)
        .service(get_jwt)
}

#[post("/logout")]
async fn logout(auth: UserAuth, pool: Data<PgPool>) -> impl Responder {
    let data = match auth.get_data() {
        None => return HttpResponse::Ok().body("not login"),
        Some(d) => d,
    };
    sqlx::query("DELETE FROM UserSessions WHERE UserId = $1")
        .bind(data.id)
        .execute(pool.as_ref())
        .await
        .unwrap();
    let mut cookie = Cookie::new("login-token", "");
    cookie.make_removal();
    HttpResponse::Ok()
        .cookie(cookie)
        .insert_header(("hx-redirect", "/"))
        .finish()
}

#[get("/register")]
async fn register_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../templates/user/register.html"))
}

#[derive(Debug, Deserialize)]
struct RegisterData {
    tag: String,
    pw: String,
    pw2: String,
}

#[post("/register")]
async fn register(
    register_data: Form<RegisterData>,
    pool: Data<PgPool>,
    rand: Data<Random>,
    pw_req: Data<Regex>,
) -> impl Responder {
    match try_register(&register_data, &pool, &rand, &pw_req).await {
        Ok(token) => HttpResponse::Ok()
            .cookie(create_login_cookie(token))
            .insert_header(("hx-redirect", "/"))
            .finish(),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[derive(Debug, FromRow)]
struct Id(i64);

async fn try_register(
    data: &RegisterData,
    pool: &PgPool,
    rand: &Random,
    pw_req: &Regex,
) -> Result<String, String> {
    if data.pw != data.pw2 {
        return Err(String::from("passwords dont match"));
    }
    if !pw_req
        .is_match(&data.pw)
        .map_err(|_| String::from("problem with regex"))?
    {
        return Err(String::from("password didn't match requirements"));
    }

    let mut token = [0u8; 16];
    rand.lock()
        .map_err(|e| e.to_string())?
        .fill_bytes(&mut token);
    let token = u128::from_le_bytes(token).to_string();
    let hash = password_auth::generate_hash(&data.pw);
    let Id(user_id) =
        sqlx::query_as("INSERT INTO UserLogins(tag, hash) VALUES ($1, $2) RETURNING id")
            .bind(&data.tag)
            .bind(&hash)
            .fetch_one(pool)
            .await
            .map_err(|_| String::from("name already exists"))?;

    sqlx::query("INSERT INTO UserSessions(token, UserId) VALUES ($1, $2)")
        .bind(&token)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(token)
}

#[get("/login")]
async fn login_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../templates/user/login.html"))
}

#[derive(Debug, Deserialize)]
struct LoginData {
    tag: String,
    pw: String,
}

#[post("/login")]
async fn login(data: Form<LoginData>, pool: Data<PgPool>, rand: Data<Random>) -> impl Responder {
    match try_login(&data, &pool, &rand).await {
        Ok(token) => HttpResponse::Ok()
            .cookie(create_login_cookie(token))
            .insert_header(("hx-redirect", "/"))
            .finish(),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[derive(Debug, FromRow)]
struct TryLogin {
    id: i64,
    hash: String,
}

impl TryLogin {
    fn test(&self, pw: &str) -> Option<()> {
        password_auth::verify_password(pw, &self.hash).ok()
    }
}

async fn try_login(data: &LoginData, pool: &PgPool, rand: &Random) -> Result<String, String> {
    let mut token = [0u8; 16];
    rand.lock()
        .map_err(|e| e.to_string())?
        .fill_bytes(&mut token);
    let token = u128::from_le_bytes(token).to_string();
    let try_login: TryLogin = sqlx::query_as("SELECT id, hash FROM UserLogins WHERE tag = $1")
        .bind(&data.tag)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    if try_login.test(&data.pw).is_none() {
        return Err(String::from("could not login"));
    }

    sqlx::query("INSERT INTO UserSessions(token, UserId) VALUES ($1, $2)")
        .bind(&token)
        .bind(try_login.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(token)
}

#[derive(Serialize)]
struct JWT {
    jwt: String,
}

#[get("/get_jwt")]
async fn get_jwt(
    pool: Data<PgPool>,
    secret: Data<Secret>,
    rand: Data<Random>,
    req: HttpRequest,
) -> impl Responder {
    match try_refresh_token(&pool, &secret, &rand, req).await {
        Some((token, jwt)) => HttpResponse::Ok()
            .cookie(create_login_cookie(token))
            .json(jwt),
        None => HttpResponse::Ok().json(JWT {
            jwt: String::from("Not a JWT"),
        }),
    }
}

async fn try_refresh_token(
    pool: &PgPool,
    secret: &Secret,
    rand: &Random,
    req: HttpRequest,
) -> Option<(String, JWT)> {
    let token = req.cookie("login-token")?.value().to_string();
    let jwt = create_jwt_for_cookie(&token, pool, secret).await?;

    let mut new_token = [0u8; 16];
    rand.lock().unwrap().fill_bytes(&mut new_token);
    let new_token = u128::from_le_bytes(new_token);
    let new_token = format!("{new_token}");

    sqlx::query("UPDATE UserSessions SET token = $2 WHERE token = $1")
        .bind(token)
        .bind(&new_token)
        .execute(pool)
        .await
        .unwrap();

    Some((new_token, JWT { jwt }))
}

fn create_login_cookie<'a>(token: String) -> Cookie<'a> {
    CookieBuilder::new("login-token", token)
        .max_age(Duration::weeks(3))
        .secure(true)
        .finish()
}
