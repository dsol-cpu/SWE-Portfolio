use actix_web::{ HttpResponse, Result, Error };
use reqwest::Client;
use serde::{ Deserialize, Serialize };
use std::env::{ self, VarError };

#[derive(Serialize, Deserialize)]
struct GithubStats {
    total_public_repos: i32,
    total_private_repos: i32,
    followers: i32,
    stars: i32,
    contributions: i32,
    top_languages: Vec<String>,
}

pub async fn get_github_stats() -> Result<HttpResponse, Error> {
    let client: Client = Client::new();

    // Get environment variables
    let github_username = match env::var("MY_GITHUB_USERNAME") {
        Ok(username) => username,
        Err(err) => {
            return Ok(
                HttpResponse::InternalServerError().body(
                    format!("Failed to fetch GitHub stats: {}", err)
                )
            );
        }
    };

    let github_api_token: String = match env::var("MY_GITHUB_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            return Ok(
                HttpResponse::InternalServerError().body(
                    format!("Failed to fetch GitHub stats: {}", err)
                )
            );
        }
    };

    // Send request to GitHub API
    let response: Result<reqwest::Response, Error> = match
        client
            .get(&format!("https://api.github.com/users/{}", github_username))
            .bearer_auth(&github_api_token)
            .send().await
    {
        Ok(resp) => Ok(resp),
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().body(
                    format!("Failed to fetch GitHub stats: {}", e)
                )
            );
        }
    };

    // Parse response
    match
        (
            match response {
                Ok(it) => it,
                Err(err) => {
                    return Err(err);
                }
            }
        ).json::<GithubStats>().await
    {
        Ok(stats) => {
            let formatted_stats = format!(
                "Public repos: {}, Private repos: {}, Followers: {}, Stars: {}, Contributions {}, Top Languages: {}",
                stats.total_public_repos,
                stats.total_private_repos,
                stats.followers,
                stats.stars,
                stats.contributions,
                stats.top_languages.join(", ")
            );
            Ok(HttpResponse::Ok().body(formatted_stats))
        }
        Err(e) =>
            Ok(
                HttpResponse::InternalServerError().body(
                    format!("Failed to parse GitHub stats: {}", e)
                )
            ),
    }
}
