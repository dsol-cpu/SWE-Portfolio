use actix_web::Responder;
use actix_web::{ get, http::StatusCode, HttpResponse };
use awc::Client;
use crate::constants::GITHUB_API_URL;
use crate::schemas::github_stats::{ Repository };
use crate::utils::github::{ create_github_client };

#[get("/api/repos")]
async fn get_user_repos() -> impl Responder {
    let client: Client = create_github_client().await;

    //first check
    match client.get(format!("{}/user/repos", GITHUB_API_URL)).send().await {
        Ok(mut response) => {
            match response.status() {
                StatusCode::OK => {
                    match response.json::<Vec<Repository>>().await {
                        Ok(repos) => HttpResponse::Ok().json(repos),
                        Err(e) =>
                            HttpResponse::InternalServerError().body(
                                format!("Failed to parse response: {}", e)
                            ),
                    }
                }
                StatusCode::UNAUTHORIZED => {
                    HttpResponse::Unauthorized().body("Invalid GitHub token")
                }
                StatusCode::FORBIDDEN => {
                    HttpResponse::Forbidden().body(
                        "Access to GitHub API is forbidden. Check rate limits or token permissions."
                    )
                }
                StatusCode::NOT_FOUND => {
                    HttpResponse::NotFound().body("GitHub resource not found")
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    HttpResponse::TooManyRequests().body("Rate limit exceeded for GitHub API")
                }
                other => {
                    HttpResponse::build(other).body(
                        format!("Unexpected error from GitHub: {}", other)
                    )
                }
            }
        }
        Err(e) =>
            HttpResponse::InternalServerError().body(
                format!("Failed to fetch repositories: {}", e)
            ),
    }
}
