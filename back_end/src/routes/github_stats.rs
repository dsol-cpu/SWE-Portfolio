use actix_web::{ get, http::StatusCode, HttpResponse, ResponseError };
use awc::Client;
use serde::{ Deserialize, Serialize };
use std::env::VarError;
use crate::lib::utils::api::{ ApiError, ApiResponse };
// GitHub data structures
#[derive(Debug, Serialize, Deserialize)]
struct GithubStats {
    login: String,
    public_repos: i32,
    followers: i32,
    following: i32,
    created_at: String,
    updated_at: String,
}

// API endpoint
#[get("/restapi/github-stats/{username}")]
pub async fn get_github_stats(
    username: actix_web::web::Path<String>,
    client: actix_web::web::Data<Client>
) -> Result<HttpResponse, ApiError> {
    let github_stats = fetch_github_data(&client, &username).await?;

    let response: ApiResponse<GithubStats> = ApiResponse {
        success: true,
        data: Some(github_stats),
        error: None,
    };

    Ok(HttpResponse::Ok().json(response))
}

// Helper functions
async fn get_github_token() -> Result<String, VarError> {
    std::env::var("GITHUB_API_TOKEN")
}

async fn fetch_github_data(client: &Client, username: &str) -> Result<GithubStats, ApiError> {
    let token = get_github_token().await?;
    let github_url = format!("https://api.github.com/users/{}", username);

    let mut response = client
        .get(&github_url)
        .insert_header(("User-Agent", "Github-Stats-App"))
        .bearer_auth(&token)
        .send().await?;

    let status = response.status();
    if !status.is_success() {
        return Err(match status {
            StatusCode::NOT_FOUND => ApiError::NotFound("GitHub user not found".to_string()),
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized("Invalid GitHub token".to_string()),
            StatusCode::FORBIDDEN => {
                if
                    let Some(remaining) = response
                        .headers()
                        .get("x-ratelimit-remaining")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<i32>().ok())
                {
                    if remaining == 0 {
                        ApiError::RateLimited("GitHub API rate limit exceeded".to_string())
                    } else {
                        ApiError::Unauthorized("Access forbidden".to_string())
                    }
                } else {
                    ApiError::Unauthorized("Access forbidden".to_string())
                }
            }
            _ => ApiError::Internal(format!("GitHub API error: {}", status)),
        });
    }

    let body_bytes = response.body().await?;
    let github_stats = serde_json::from_slice(&body_bytes)?;
    Ok(github_stats)
}

// Configuration function
pub fn configure_services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_github_stats);
}
