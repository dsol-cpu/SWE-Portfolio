use std::env;
use actix_web::Responder;
use actix_web::{ get, http::StatusCode, HttpResponse, ResponseError };
use awc::http::header::{ ACCEPT, AUTHORIZATION, USER_AGENT };
use awc::Client;
use crate::lib::schemas::github_stats::{ Repository, CachedRepository };

fn create_github_client() -> Client {
    let github_token = env::var("GITHUB_API_TOKEN").expect("GITHUB_TOKEN must be set");

    // Create the header tuples
    let auth_header = (AUTHORIZATION, format!("Bearer {}", github_token));
    let accept_header = (ACCEPT, "application/vnd.github.v3+json");
    let user_agent_header = (USER_AGENT, "rust-github-stats");

    // Build the client with headers
    Client::builder()
        .add_default_header(auth_header)
        .add_default_header(accept_header)
        .add_default_header(user_agent_header)
        .finish()
}

#[get("/repos")]
async fn get_user_repos() -> impl Responder {
    let client = create_github_client();

    //first check
    match client.get("https://api.github.com/user/repos").send().await {
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
