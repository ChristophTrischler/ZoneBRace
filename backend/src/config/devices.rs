use actix_web::web::{Data, Form};
use actix_web::{get, post};
use actix_web::{HttpResponse, Responder, Scope};
use askama::Template;
use rand_core::RngCore;
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::auth::UserAuth;
use crate::Random;

pub fn get_device_scope() -> Scope {
    Scope::new("/devices")
        .service(create_new_device)
        .service(get_devices)
}

#[derive(Debug, Template)]
#[template(path = "config/devices.html")]
struct DeviceHTML {
    rows: Vec<Row>,
    trails: Vec<Row>,
}

#[derive(sqlx::FromRow, Debug)]
struct Row {
    id: i32,
    name: String,
}

#[get("")]
async fn get_devices(pool: Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as(
        "SELECT d.id, t.name FROM Devices as d JOIN Trails as t ON d.trailId = t.id",
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap();

    let trails = sqlx::query_as("SELECT id, name FROM Trails")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body(DeviceHTML { rows, trails }.render().unwrap())
}

#[derive(Debug, Deserialize)]
struct DeviceData {
    trail_id: i64,
}

#[post("/new")]
async fn create_new_device(
    auth: UserAuth,
    data: Form<DeviceData>,
    pool: Data<PgPool>,
    rand: Data<Random>,
) -> impl Responder {
    match auth {
        UserAuth::Admin(_) => (),
        _ => return HttpResponse::Ok().body("Unauthorized"),
    }
    match try_create_new_trail(&data, &pool, &rand).await {
        Ok(token) => HttpResponse::Ok().body(format!("created device with token={token}")),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

async fn try_create_new_trail(
    data: &DeviceData,
    pool: &PgPool,
    rand: &Random,
) -> Result<String, String> {
    let mut token_buf = [0u8; 8];
    rand.lock().unwrap().fill_bytes(&mut token_buf);
    let token = u64::from_le_bytes(token_buf).to_string();
    let res = sqlx::query("INSERT INTO Devices(TrailId, token) VALUES($1, $2)")
        .bind(data.trail_id)
        .bind(&token)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    println!("{:?}", res);
    Ok(token)
}
