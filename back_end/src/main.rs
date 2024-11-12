use std::env;
use actix_web::{ web, App, HttpServer };
use actix_cors::Cors;
use deadpool::managed::QueueMode;
use deadpool_postgres::{ tokio_postgres::NoTls, Config, PoolConfig, Runtime, Timeouts };
use dotenv::dotenv;

mod lib;
mod routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize database pool
    let pool = create_supabase_pool_deadpool().await.expect("Failed to create database pool");

    let port = std::env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("*") // Replace with your actual allowed origin
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(routes::page_stats::get_rest_api_page_stats)
    })
        .bind(("0.0.0.0", port))?
        .run().await
}

async fn create_supabase_pool_deadpool() -> Result<
    deadpool_postgres::Pool,
    Box<dyn std::error::Error>
> {
    dotenv().ok();

    // Get Supabase connection details from environment variables
    let db_host = env::var("SUPABASE_DB_HOST")?;
    let db_port = env::var("SUPABASE_DB_PORT")?.parse::<u16>()?;
    let db_name = env::var("SUPABASE_DB_NAME")?;
    let db_user = env::var("SUPABASE_DB_USER")?;
    let db_password = env::var("SUPABASE_DB_PASSWORD")?;

    let mut cfg = Config::new();
    cfg.host = Some(db_host);
    cfg.port = Some(db_port);
    cfg.dbname = Some(db_name);
    cfg.user = Some(db_user);
    cfg.password = Some(db_password);

    // Supabase-specific settings
    cfg.ssl_mode = Some(deadpool_postgres::SslMode::Require); // Supabase requires SSL

    // Configure pool settings with queue_mode
    cfg.pool = Some(PoolConfig {
        max_size: 16,
        queue_mode: QueueMode::Lifo,
        timeouts: Timeouts {
            wait: Some(std::time::Duration::from_secs(15)),
            create: Some(std::time::Duration::from_secs(5)),
            recycle: Some(std::time::Duration::from_secs(5)),
        },
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}
