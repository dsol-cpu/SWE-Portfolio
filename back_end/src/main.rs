use std::env;

use actix_web::{ web, App, HttpServer };
use crate::utils::supabase::init_database;
mod utils;
mod routes;
mod constants;
mod types;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Initialize the database connection pool
    let config = init_database().await.expect("Failed to initialize database pool");

    // Read the PORT environment variable or default to 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    println!("Starting server at http://{}", address);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(config.clone())).configure(routes::configure)
    })
        .bind(address)?
        .run().await
}
