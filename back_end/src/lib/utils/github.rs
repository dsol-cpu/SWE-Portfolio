use std::env::VarError;
use awc::{ http::StatusCode, Client };
use chrono::Utc;
use serde::Deserialize;

use crate::lib::{ schemas::github_stats::Repository, types::error::ApiError };
// use super::api::ApiError;

// Add this struct to handle GitHub API response
#[derive(Deserialize)]
pub struct GithubApiResponse {
    pub name: String,
    pub updated_at: String,
    pub pushed_at: String,
}

pub async fn fetch_github_data(client: &Client, username: &str) -> Result<Repository, ApiError> {
    let token = std::env::var("GITHUB_API_TOKEN").map_err(|e| ApiError::Internal(e.to_string()))?;
    let github_url = format!("https://api.github.com/users/{}", username);

    // Use client.get() to create the request
    let request = client
        .get(&github_url)
        .insert_header(("User-Agent", "Github-Stats-App"))
        .bearer_auth(&token);

    // Send the request and await the response
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

    // Parse the response
    let github_data: GithubApiResponse = response
        .json().await
        .map_err(|e| ApiError::Internal(format!("Failed to parse GitHub response: {}", e)))?;

    // Convert to your Repository struct
    let repository = Repository {
        name: github_data.name,
        updated_at: github_data.updated_at,
        pushed_at: github_data.pushed_at,
        cached: false, // This is coming directly from API
        cache_age: Utc::now(), // Current time as it's fresh data
    };

    Ok(repository)
}

// Helper function remains the same
pub async fn get_github_token() -> Result<String, VarError> {
    std::env::var("GITHUB_API_TOKEN")
}
