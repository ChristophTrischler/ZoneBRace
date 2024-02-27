use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{error::ErrorUnauthorized, web::Data, FromRequest, HttpRequest};
use jsonwebtoken::{DecodingKey, EncodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::Secret;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserData {
    pub id: i64,
    pub tag: String,
    pub permissions: i32,
    pub exp: i32,
}

#[derive(Debug)]
pub enum UserAuth {
    User(UserData),
    Admin(UserData),
    UnAuthorized,
}

impl UserAuth {
    pub fn get_data(&self) -> Option<&UserData> {
        match self {
            UserAuth::User(data) => Some(data),
            UserAuth::Admin(data) => Some(data),
            UserAuth::UnAuthorized => None,
        }
    }

    pub fn pop_data(self) -> Option<UserData> {
        match self {
            UserAuth::User(data) => Some(data),
            UserAuth::Admin(data) => Some(data),
            UserAuth::UnAuthorized => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            UserAuth::Admin(_) => true,
            _ => false,
        }
    }
}

impl FromRequest for UserAuth {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match jwt_to_userdata(req) {
            None => ready(Ok(Self::UnAuthorized)),
            Some(auth) => ready(Ok(auth)),
        }
    }
}

fn jwt_to_userdata(req: &HttpRequest) -> Option<UserAuth> {
    let jwt = req.headers().get("jwt")?;
    let secret: &Data<Secret> = req.app_data()?;

    let data: TokenData<UserData> = jsonwebtoken::decode(
        jwt.to_str().ok()?,
        &DecodingKey::from_secret(secret.as_slice()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .ok()?;

    let user = UserData::from(data.claims);

    match user.permissions {
        0 => Some(UserAuth::User(user)),
        _ => Some(UserAuth::Admin(user)),
    }
}

#[derive(Debug, FromRow)]
pub struct DeviceAuth {
    pub id: i32,
    pub trailid: i32,
}

impl FromRequest for DeviceAuth {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match try_device_auth(&req).await {
                Some(auth) => Ok(auth),
                None => Err(ErrorUnauthorized("wrong token")),
            }
        })
    }
}

async fn try_device_auth(req: &HttpRequest) -> Option<DeviceAuth> {
    let pool: &Data<PgPool> = req.app_data()?;
    let token = req.headers().get("token")?;
    sqlx::query_as("SELECT id, trailid FROM Devices WHERE token = $1")
        .bind(token.to_str().unwrap())
        .fetch_one(pool.as_ref())
        .await
        .ok()
}

pub async fn create_jwt_for_cookie(token: &str, pool: &PgPool, secret: &Secret) -> Option<String> {
    let user: UserData = sqlx::query_as(
        "SELECT u.id, u.tag, u.permissions, 
        extract( epoch from (current_date + '24 hours'::interval))::integer as exp 
        FROM UserLogins as u JOIN UserSessions as s ON u.id = s.UserId 
        WHERE token = $1",
    )
    .bind(token)
    .fetch_one(pool)
    .await
    .ok()?;

    let jwt = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user,
        &EncodingKey::from_secret(secret.as_slice()),
    )
    .ok()?;
    Some(jwt)
}
