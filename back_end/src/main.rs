use std::env;

use actix_web::{ middleware, web, App, HttpServer };
use crate::utils::supabase::init_database;
mod utils;
mod routes;
mod constants;
mod types;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize the database connection pool
    let config = init_database().await.expect("Failed to initialize database pool");

    // Read the PORT environment variable or default to 8080
    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let address = format!("0.0.0.0:{}", port);
    println!("Starting server at http://{}", address);

    // Create and start the server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Add logging middleware
            .app_data(web::Data::new(config.clone()))
            .configure(routes::configure)
    })
        .workers(2) // Specify number of workers
        .bind(&address)?
        .run().await
}
