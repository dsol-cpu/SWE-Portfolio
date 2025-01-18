use std::env;
use actix_web::{ get, web, HttpResponse, Responder };
use crate::constants::{ GITHUB_API_TOKEN, GITHUB_USERNAME, REPOSITORIES };
use crate::utils::github::create_github_client;

pub fn github_stats_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(get_user_repos));
}

#[get("/github_repos")]
pub async fn get_user_repos() -> impl Responder {
    // let github_token = match env::var(GITHUB_API_TOKEN) {
    //     Ok(token) => token,
    //     Err(e) => {
    //         return HttpResponse::InternalServerError().body(format!("Failed to get token: {}", e));
    //     }
    // };

    // // Create the client first, outside the loop
    // let client = match create_github_client(github_token).await {
    //     Ok(client) => client,
    //     Err(e) => {
    //         return HttpResponse::InternalServerError().body(
    //             format!("Failed to create GitHub client: {}", e)
    //         );
    //     }
    // };

    // let mut repos_data = Vec::new();

    // // Fetch repositories and collect data
    // for repo in REPOSITORIES.iter().filter(|r| !r.is_empty()) {
    //     match client.get_repository(GITHUB_USERNAME, repo).await {
    //         Ok(repo) => repos_data.push(repo),
    //         Err(e) => {
    //             // Instead of returning an error here, we log it and continue processing the rest
    //             eprintln!("Failed to fetch repository {}: {:#?}", repo, e);
    //         }
    //     }
    // }

    // Return all fetched repository data or an empty response if none were fetched
    // if repos_data.is_empty() {
    //     HttpResponse::NotFound().body("No repositories found or failed to fetch all.")
    // } else {
    HttpResponse::Ok().json("okay!")
    // }
}
