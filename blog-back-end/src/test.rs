#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::fs;

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

    #[actix_rt::test]
    async fn test_1() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let pg_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

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
        .fetch_all(&pg_pool)
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
        println!("{}", result);

        let target: String = String::from("postgres://postgres:ZCzc960830@localhost:5432/blog");
        assert_eq!(database_url.eq(&target), true);
    }

    #[test]
    fn test_2() {
        let path = "/home/ubuntu/workarea/code/blog-back-end/README.md";
        let article = fs::read_to_string(path).unwrap();
        println!("{}", article);
    }
}
