use std::env;
use awc::http::header::{ ACCEPT, AUTHORIZATION, USER_AGENT };
use awc::{ http::StatusCode, Client };
use chrono::Utc;
use serde::Deserialize;
use crate::constants::{ GITHUB_API_TOKEN, GITHUB_API_URL, GITHUB_USERNAME };
use crate::lib::{ schemas::github_stats::Repository, types::error::ApiError };

#[derive(Deserialize)]
pub struct GithubApiResponse {
    name: String,
    updated_at: String,
    pushed_at: String,
}

pub async fn create_github_client() -> Client {
    let github_token = env::var(GITHUB_API_TOKEN).expect("GITHUB_TOKEN must be set");

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

pub async fn fetch_github_data(client: &Client) -> Result<Repository, ApiError> {
    let github_token = env::var(GITHUB_API_TOKEN).map_err(|e| ApiError::Internal(e.to_string()))?;
    let github_api_url: String = env
        ::var(GITHUB_API_URL)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let github_username: String = env
        ::var(GITHUB_USERNAME)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let github_url = format!("{}/users/{}", github_api_url, github_username);

    let mut request = client.get(&github_url);
    request = request.insert_header((AUTHORIZATION, format!("token {}", github_token)));
    request = request.insert_header((USER_AGENT, "rust-github-client"));

    let mut response = request.send().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let status = response.status();

    if !status.is_success() {
        return Err(match status {
            StatusCode::NOT_FOUND => ApiError::NotFound("GitHub user not found".to_string()),
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized("Invalid GitHub token".to_string()),
            StatusCode::FORBIDDEN => {
                let remaining = response
                    .headers()
                    .get("x-ratelimit-remaining")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<i32>().ok());

                match remaining {
                    Some(0) => ApiError::RateLimited("GitHub API rate limit exceeded".to_string()),
                    _ => ApiError::Unauthorized("Access forbidden".to_string()),
                }
            }
            _ => ApiError::Internal(format!("GitHub API error: {}", status)),
        });
    }

    let github_data: GithubApiResponse = response
        .json().await
        .map_err(|e| ApiError::Internal(format!("Failed to parse GitHub response: {}", e)))?;

    let repository = Repository {
        name: github_data.name,
        updated_at: github_data.updated_at,
        pushed_at: github_data.pushed_at,
        cached: false,
        cache_age: Utc::now(),
    };

    Ok(repository)
}
