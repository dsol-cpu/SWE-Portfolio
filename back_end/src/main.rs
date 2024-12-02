use std::env;
use constants::{ HOST_PLATFORM_ADDRESS, PORT };
use env_logger;
use actix_governor::{ Governor, GovernorConfigBuilder };
use actix_web::{ middleware::Logger, web, App, HttpServer };
use types::error::ApiError;
use crate::utils::supabase::init_database;

mod utils;
mod api;
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
    let supabase_client = init_database().await;

    // Create a Quota for rate limiting (1 request per 2 seconds with burst of 5)
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let address = env
        ::var(HOST_PLATFORM_ADDRESS)
        .map_err(|e| ApiError::ConfigError(format!("Missing HOST_PLATFORM_ADDRESS: {}", e)))?;

    let port = env
        ::var(PORT)
        .map_err(|e| ApiError::ConfigError(format!("Missing PORT: {}", e)))?
        .parse::<u16>()
        .map_err(|e| ApiError::ConfigError(format!("Invalid PORT value: {}", e)))?;

    // Log server startup
    log::info!("starting HTTP server at http://{address}:{port}");

    HttpServer::new(move || {
        log::debug!("Constructing the App");
        let governor = Governor::new(&governor_conf);
        App::new().wrap(Logger::default()).wrap(governor).configure(api::configure)
    })
        .bind((address, port))?
        .workers(2)
        .run().await
}
