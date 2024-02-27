use actix_web::web::{Data, Form};
use actix_web::{get, post};
use actix_web::{HttpResponse, Responder, Scope};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::auth::UserAuth;

pub fn get_trail_scope() -> Scope {
    Scope::new("/trails")
        .service(create_new_trail)
        .service(get_trails)
}

#[derive(Debug, Template)]
#[template(path = "config/trails.html")]
struct TrailHTML {
    rows: Vec<Row>,
}

#[derive(sqlx::FromRow, Debug)]
struct Row {
    id: i32,
    name: String,
    len: f64,
}

#[get("")]
async fn get_trails(pool: Data<PgPool>) -> impl Responder {
    let rows = sqlx::query_as("SELECT id, name, len FROM Trails")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body(TrailHTML { rows }.render().unwrap())
}

#[derive(Debug, Deserialize)]
struct TrailData {
    name: String,
    len: f64,
}

#[post("/new")]
async fn create_new_trail(
    auth: UserAuth,
    data: Form<TrailData>,
    pool: Data<PgPool>,
) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Ok().body("no permission to create Device");
    }
    match try_create_new_trail(&data, &pool).await {
        Ok(_) => HttpResponse::Ok().body("created Device"),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

async fn try_create_new_trail(data: &TrailData, pool: &PgPool) -> Result<(), String> {
    let _res = sqlx::query("INSERT INTO Trails(name,len) VALUES($1, $2)")
        .bind(&data.name)
        .bind(data.len)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
