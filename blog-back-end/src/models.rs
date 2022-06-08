use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetRequest {
    pub key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Subject {
    pub key: String,
    pub a_0: Vec<String>,
    pub a_1: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub articles: Vec<Article>,
}

impl Category {
    pub fn new(name: String, articles: Vec<Article>) -> Category {
        Category {
            name: name,
            articles: articles,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub name: String,
    pub timestamp: String,
}

impl Article {
    pub fn new(name: String, timestamp: String) -> Article {
        Article {
            name: name,
            timestamp: timestamp,
        }
    }
}
