use std::env;
use env_logger;
use actix_web::{ middleware::Logger, web, App, HttpServer };

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

    // Initialize logger with default "info" level
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize the database connection pool
    // let config = init_database().await.expect("Failed to initialize database pool");

    // Log server startup
    log::info!("starting HTTP server at http://0.0.0.0:10000");

    HttpServer::new(move || {
        log::debug!("Constructing the App");

        App::new()
            // .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .configure(routes::configure)
    })
        .bind(("0.0.0.0", 10000))?
        .workers(2)
        .run().await
}
