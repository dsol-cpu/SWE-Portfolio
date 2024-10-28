use actix_web::{ web, HttpResponse, Result };
use chrono::{ DateTime, Utc };
use serde::Serialize;

#[derive(Serialize)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    published_at: DateTime<Utc>,
    tags: Vec<String>,
}

pub async fn get_blog_posts() -> Result<HttpResponse> {
    // Fetch from database (Supabase)
    let posts = vec![BlogPost {
        id: 1,
        title: "First Post".to_string(),
        content: "Content here".to_string(),
        published_at: Utc::now(),
        tags: vec!["rust".to_string(), "programming".to_string()],
    }];

    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_blog_post(path: web::Path<i32>) -> Result<HttpResponse> {
    let post_id = path.into_inner();
    // Fetch specific post from Supabase
    Ok(HttpResponse::Ok().json("Blog post details"))
}
