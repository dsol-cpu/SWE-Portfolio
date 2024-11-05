use actix_web::{ web, App, HttpServer };
use actix_cors::Cors;
use routers::github_stats::{
    get_github_repo,
    get_github_repo_languages,
    get_github_repo_stats,
    get_github_repos,
};

use dotenv::dotenv;

mod routers;
// Main function to configure and start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize database pool
    let pool = init_db_pool().await;

    // Initialize schema
    let client = pool.get().await.expect("Failed to get DB client");
    init_db(&client).await.expect("Failed to initialize database");

    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://your-username.github.io")
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(get_page_stats)
            .service(get_popular_pages)
    })
        .bind(("0.0.0.0", port))?
        .run().await
}
