use super::models::*;
use super::state::AppState;
use actix_web::{web, Responder};
use std::fs;
use chrono::{DateTime, Utc};

pub async fn subject_get(
    info: web::Query<GetRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    println!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    format!("key: {}", info.key)
}

pub async fn subject_set(info: web::Json<Subject>) -> impl Responder {
    format!("{}", serde_json::to_string(&info).unwrap())
}

pub async fn subject_del(info: web::Query<GetRequest>) -> impl Responder {
    format!("key: {}", info.key)
}

pub async fn content_get(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    println!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    let pg_pool = &app_state.db;
    let results = sqlx::query!(
        r#"
        SELECT 
            c.name as category, 
            a.name as name, 
            a.timestamp as timestamp 
        FROM 
            category as c 
        INNER JOIN 
            article as a 
        ON 
            c.name = a.category;
        "#
    )
    .fetch_all(pg_pool)
    .await
    .unwrap();

    let mut content: Vec<Category> = vec![];

    for r in results {
        let category_name = r.category.unwrap();
        let article_name = r.name.unwrap();
        let timestamp = r.timestamp.unwrap().to_string();
        let len = content.len();
        let article = Article::new(article_name, timestamp);

        if len == 0 {
            content.push(Category::new(category_name, vec![article]));
        } else {
            match content.get_mut(len - 1) {
                Some(category) => {
                    if category.name.eq(&category_name) {
                        category.articles.push(article);
                    } else {
                        content.push(Category::new(category_name, vec![article]));
                    }
                }
                _ => (),
            }
        }
    }

    let result = serde_json::to_string(&content).unwrap();
    format!("{}", result)
}

pub async fn article_get(app_state: web::Data<AppState>) -> impl Responder {
    println!("article_get");

    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    println!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    let path = "/home/ubuntu/workarea/code/blog-back-end/README.md";
    let article = fs::read_to_string(path).unwrap();

    println!("{}", article);

    format!("{}", article)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/subject/get", web::get().to(subject_get));
    cfg.route("/subject/set", web::post().to(subject_set));
    cfg.route("/subject/del", web::post().to(subject_del));
    cfg.route("/content/get", web::get().to(content_get));
    cfg.route("/article/get", web::get().to(article_get));
}
