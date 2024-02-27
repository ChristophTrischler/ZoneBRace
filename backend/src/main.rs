use std::sync::{Arc, Mutex};

use actix_files::Files;
use actix_web::{
    get,
    middleware::{self},
    web::{self, Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use askama::Template;
use data::que::RiderQue;
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Executor, PgPool};

mod auth;
use auth::auth::UserAuth;
mod config;
mod data;

type Random = Mutex<ChaCha8Rng>;
type Secret = [u8; 16];

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("could not create schema");

    let rand = Arc::new(Mutex::new(ChaCha8Rng::seed_from_u64(
        rand_core::OsRng.next_u64(),
    )));

    let mut secret: Secret = Default::default();
    rand.lock().unwrap().fill_bytes(&mut secret);

    let que = Arc::new(Mutex::new(RiderQue::new()));

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(Files::new("/static", "static"))
                .service(reload_page)
                .service(index)
                .service(user_info)
                .service(get_links)
                .service(data::que::get_que_scope())
                .service(config::trails::get_trail_scope())
                .service(config::devices::get_device_scope())
                .service(data::scoreboard::get_scoreboard_scope())
                .service(data::data::get_data_scope())
                .service(auth::user::get_user_scope())
                .wrap(middleware::Logger::default()),
        )
        .app_data(Data::new(pool))
        .app_data(Data::from(rand))
        .app_data(Data::from(que))
        .app_data(Data::new(secret));
    };

    Ok(config.into())
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexHtml {
    page: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        IndexHtml {
            page: String::from("links"),
        }
        .render()
        .unwrap(),
    )
}

#[derive(Template)]
#[template(path = "user/user.html")]
struct UserDiv<'a> {
    tag: &'a str,
}

#[get("/user")]
async fn user_info(auth: UserAuth) -> impl Responder {
    match auth.get_data() {
        None => HttpResponse::Ok().body(include_str!("../templates/user/not_loged_in.html")),
        Some(user_data) => HttpResponse::Ok().body(
            UserDiv {
                tag: user_data.tag.as_str(),
            }
            .render()
            .unwrap(),
        ),
    }
}
#[get("/home/{page}")]
async fn reload_page(page: Path<String>) -> impl Responder {
    let page = page.clone();
    HttpResponse::Ok().body(IndexHtml { page }.render().unwrap())
}

#[derive(Template)]
#[template(path = "link_list.html")]
struct LinkList<'a> {
    links: Vec<&'a str>,
}

#[get("/links")]
async fn get_links(auth: UserAuth) -> impl Responder {
    let links = match auth {
        UserAuth::Admin(_) => vec!["scoreboard", "que", "trails", "devices"],
        _ => vec!["scoreboard", "que"],
    };
    HttpResponse::Ok().body(LinkList { links }.render().unwrap())
}
