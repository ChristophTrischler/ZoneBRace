use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder, Scope,
};
use askama::Template;
use sqlx::PgPool;

pub fn get_scoreboard_scope() -> Scope {
    Scope::new("/scoreboard")
        .service(get_results)
        .service(get_table)
}

#[derive(Template)]
#[template(path = "data/table.html")]
struct TabelHTML {
    rows: Vec<Row>,
}

#[derive(Debug, sqlx::FromRow)]
struct Row {
    tag: String,
    name: String,
    start: String,
    finish: String,
    time: String,
}

#[derive(serde::Deserialize)]
struct TableParameters {
    rider: Option<String>,
    trail: Option<String>,
}

#[get("/table")]
async fn get_table(pool: Data<PgPool>, para: Query<TableParameters>) -> impl Responder {
    match sqlx::query_as(
        "SELECT tag, name, 
            COALESCE(to_char(start_time, 'DD.MM.YYYY HH24:MI:SS'),'') as start,
            COALESCE(to_char(finish_time, 'DD.MM.YYYY HH24:MI:SS'),'') as finish,
            COALESCE(to_char(finish_time - start_time,'MI:SS'),'') as time 
            FROM UserLogins as u
            JOIN Runs as r ON u.id = r.UserId
            JOIN Trails as t ON t.id = r.TrailId 
            WHERE t.name like COALESCE(NULLIF($1, ''), '%')
            AND tag like COALESCE(NULLIF($2,''), '%')
            ORDER BY finish_time - start_time, start_time",
    )
    .bind(&para.trail)
    .bind(&para.rider)
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().body(TabelHTML { rows }.render().unwrap()),
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}

#[derive(Template)]
#[template(path = "data/scoreboard.html")]
struct ScoreboardSite {
    rider: Vec<String>,
    trails: Vec<String>,
}

#[get("")]
async fn get_results(pool: Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body(get_scoreboardsite(&pool).await.unwrap().render().unwrap())
}

#[derive(sqlx::FromRow)]
struct Name(String);
impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}

async fn get_scoreboardsite(pool: &PgPool) -> Result<ScoreboardSite, sqlx::Error> {
    let rider: Vec<Name> = sqlx::query_as("SELECT tag FROM UserLogins")
        .fetch_all(pool)
        .await?;
    let trails: Vec<Name> = sqlx::query_as("SELECT name FROM trails")
        .fetch_all(pool)
        .await?;
    let rider = rider.into_iter().map(Name::into).collect();
    let trails = trails.into_iter().map(Name::into).collect();
    Ok(ScoreboardSite { rider, trails })
}
