use std::env;
use actix_web::{ get, Responder, HttpResponse };
use utoipa::OpenApi;
use crate::constants::{ GITHUB_API_TOKEN, GITHUB_USERNAME };
use crate::utils::github::create_github_client;

const REPOSITORIES: [&str; 5] = [
    "SWE-Portfolio",
    "MK-AREAS-plugin",
    "GeospatialDataVisualization-Aug2024",
    "MMORG-DB",
    "alien-search",
];

pub struct ApiDoc;

#[get("/api/github_repos")]
async fn get_user_repos() -> impl Responder {
    let github_token = match env::var(GITHUB_API_TOKEN) {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to get token: {}", e));
        }
    };

    let client = create_github_client(github_token);

    let mut repos_data = Vec::new();

    for repo in REPOSITORIES.iter().filter(|r| !r.is_empty()) {
        match client.get_repository(GITHUB_USERNAME, repo).await {
            Ok(repo) => repos_data.push(repo),
            Err(e) => {
                return HttpResponse::InternalServerError().body(
                    format!("Failed to fetch repository {}: {:#?}", repo, e)
                );
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!(repos_data))
}
