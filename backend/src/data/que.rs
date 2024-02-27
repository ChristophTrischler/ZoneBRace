use std::sync::Mutex;

use crate::auth::auth::{DeviceAuth, UserAuth};
use actix_web::{get, post, web::Data, HttpResponse, Responder, Scope};
use askama::Template;
use sqlx::{FromRow, PgPool};

#[derive(Debug)]
pub struct RiderQue {
    que: Vec<(i64, String)>,
    current: Option<(i64, String, bool)>,
}

impl RiderQue {
    pub fn new() -> Self {
        RiderQue {
            que: Vec::with_capacity(20),
            current: None,
        }
    }

    fn add(&mut self, id: i64, tag: String) {
        let element = (id, tag);
        if !self.que.contains(&element) {
            self.que.push(element);
        }
    }

    fn pop(&mut self) -> Option<(&i64, &String)> {
        if !self.que.is_empty() {
            let (id, tag) = self.que.remove(0);
            self.current = Some((id, tag, false));
            let (id, tag, _) = self.current.as_ref().unwrap();
            Some((id, tag))
        } else {
            None
        }
    }

    fn remove(&mut self, id: &i64) {
        let deletes: Vec<usize> = self
            .que
            .iter()
            .enumerate()
            .filter(|(_, (x, _))| *x == *id)
            .map(|(i, _)| i)
            .collect();
        for i in deletes {
            self.que.remove(i);
        }
    }

    pub fn set_status(&mut self, status: bool) {
        if let Some((_, _, s)) = &mut self.current {
            *s = status;
        }
    }

    pub fn remove_current(&mut self) {
        self.current = None;
    }
}

pub fn get_que_scope() -> Scope {
    Scope::new("/que")
        .service(add_to_que)
        .service(remove_from_que)
        .service(pop_from_que)
        .service(get_que)
}

#[post("/add")]
async fn add_to_que(auth: UserAuth, que: Data<Mutex<RiderQue>>) -> impl Responder {
    let data = match auth.pop_data() {
        None => return HttpResponse::Ok().body("You have to <a href='/login'>login</a> first"),
        Some(d) => d,
    };
    que.lock().unwrap().add(data.id, data.tag);
    HttpResponse::Ok().body("qued")
}

#[post("/cancel")]
async fn remove_from_que(auth: UserAuth, que: Data<Mutex<RiderQue>>) -> impl Responder {
    let data = match auth.get_data() {
        None => return HttpResponse::Ok().body("You have to <a href='/login'>login</a> first"),
        Some(d) => d,
    };
    que.lock().unwrap().remove(&data.id);
    HttpResponse::Ok().body("removed")
}

#[derive(Debug, FromRow)]
struct Id(i32);

#[get("/pop")]
async fn pop_from_que(
    auth: DeviceAuth,
    que: Data<Mutex<RiderQue>>,
    pool: Data<PgPool>,
) -> impl Responder {
    match gen_new_run(auth, que, &pool).await {
        Some(id) => HttpResponse::Ok().body(format!("{}", id.0)),
        None => HttpResponse::Ok().finish(),
    }
}

async fn gen_new_run(auth: DeviceAuth, que: Data<Mutex<RiderQue>>, pool: &PgPool) -> Option<Id> {
    let mut que = que.lock().ok()?;
    let (user_id, _) = que.pop()?;
    sqlx::query_as("INSERT INTO Runs (UserId, TrailId) VALUES ($1, $2) RETURNING id")
        .bind(user_id)
        .bind(auth.trailid)
        .fetch_one(pool)
        .await
        .ok()
}

#[derive(Debug, Template)]
#[template(path = "data/que.html")]
struct QueHtml<'a> {
    qued: bool,
    tags: Vec<&'a str>,
    current: Option<(&'a str, &'a str)>,
}

#[get("")]
async fn get_que(user_auth: UserAuth, que: Data<Mutex<RiderQue>>) -> impl Responder {
    let que = &que.lock().unwrap();
    let tags = que.que.iter().map(|(_, s)| s.as_str()).collect();
    let ids: Vec<i64> = que.que.iter().map(|(i, _)| *i).collect();
    let user = user_auth.pop_data();
    let current = que.current.as_ref().map(|(_, tag, status)| {
        (
            tag.as_str(),
            if *status {
                "currently racing"
            } else {
                "ready to start"
            },
        )
    });
    let qued = user.map(|u| ids.contains(&u.id)).unwrap_or(false);
    HttpResponse::Ok().body(
        QueHtml {
            qued,
            tags,
            current,
        }
        .render()
        .unwrap(),
    )
}
