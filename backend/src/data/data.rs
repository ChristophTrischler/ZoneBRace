use std::sync::Mutex;

use actix_web::{
    post,
    web::{Data, Form},
    HttpResponse, Responder, Scope,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::auth::DeviceAuth;
use crate::data::que::RiderQue;

pub fn get_data_scope() -> Scope {
    Scope::new("/data")
        .service(post_start_time)
        .service(post_finish_time)
}

#[derive(Debug, Deserialize)]
struct TimeData {
    time: i64,
    id: i32,
}

#[post("/start")]
async fn post_start_time(
    _auth: DeviceAuth,
    pool: Data<PgPool>,
    data: Form<TimeData>,
    que: Data<Mutex<RiderQue>>,
) -> impl Responder {
    que.lock().unwrap().set_status(true);
    let date = chrono::DateTime::from_timestamp(data.time, 0).unwrap();
    match sqlx::query("UPDATE Runs SET start_time = $1 WHERE id = $2")
        .bind(date)
        .bind(data.id)
        .execute(pool.as_ref())
        .await
    {
        Ok(r) => HttpResponse::Ok().body(r.rows_affected().to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/finish")]
async fn post_finish_time(
    _auth: DeviceAuth,
    pool: Data<PgPool>,
    data: Form<TimeData>,
    que: Data<Mutex<RiderQue>>,
) -> impl Responder {
    que.lock().unwrap().remove_current();
    let date = chrono::DateTime::from_timestamp(data.time, 0).unwrap();
    match sqlx::query("UPDATE Runs SET finish_time = $1 WHERE id = $2")
        .bind(date)
        .bind(data.id)
        .execute(pool.as_ref())
        .await
    {
        Ok(r) => HttpResponse::Ok().body(r.rows_affected().to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
