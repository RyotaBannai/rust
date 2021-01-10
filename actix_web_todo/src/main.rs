#[macro_use]
extern crate log;
mod app_errors;
use actix_web::{get, http::header, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use app_errors::errors::AppError;
use askama::Template;
use async_std::task;
use env_logger;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use std::env;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

#[derive(Deserialize)] // params: web::Form<AddParams> で自動で deserialize させる
struct AddParams {
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, AppError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?; // unwrap

    let mut entries: Vec<TodoEntry> = Vec::new();
    for row in rows {
        entries.push(row?); // unwrap
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, AppError> {
    debug!("add request");
    let conn = db.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish()) // Ok(...) でトップドメイン / へリダイレクト
}

#[post("/delete")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, AppError> {
    debug!("delete request");
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id=?", &[params.id])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool.get().expect("Failed to get the connection pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT NOT NULL
    )",
        params![],
    )
    .expect("Failed to create a table `todo`.");

    env::set_var("RUST_LOG", "actix_web_todo=debug,actix_web=info");
    env_logger::init();

    // ここでコネクションプールを渡す
    let app = move || {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    };
    debug!("app started..");
    HttpServer::new(app).bind("0.0.0.0:8081")?.run().await?;
    Ok(())
}
