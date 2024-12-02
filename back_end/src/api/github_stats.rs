use std::env;
use actix_web::{ get, http::StatusCode, Responder, HttpResponse };
use crate::constants::{ GITHUB_API_TOKEN, GITHUB_USERNAME };
use crate::schemas::github_stats::Repository;
use crate::types::error::ApiError;
use crate::utils::github::create_github_client;

#[get("/api/repos")]
async fn get_user_repos() -> impl Responder {
    let github_token = match env::var(GITHUB_API_TOKEN) {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to get token: {}", e));
        }
    };

    let client = create_github_client(github_token);

    match client.get_repository(GITHUB_USERNAME, "repo_name").await {
        Ok(repo) => HttpResponse::Ok().json(repo),
        Err(e) =>
            HttpResponse::InternalServerError().body(
                format!("Failed to fetch repository: {:#?}", e)
            ),
    }
}
