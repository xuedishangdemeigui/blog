use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io::Result as IoResult;
use std::sync::Mutex;

mod handlers;
mod models;
mod state;
mod test;

#[actix_web::main]
async fn main() -> IoResult<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pg_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let share_data = web::Data::new(state::AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        db: pg_pool,
    });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://192.168.137.3:3000")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(share_data.clone())
            .configure(handlers::config)
    };

    HttpServer::new(app).bind(("0.0.0.0", 8080))?.run().await
}
