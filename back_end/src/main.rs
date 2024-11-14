use actix_web::{ web, App, HttpServer };
use lib::utils::supabase::init_database;

mod lib;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize the database connection pool
    let pool = init_database().await.expect("Failed to initialize database pool");

    println!("Starting server at http://127.0.0.1:8080");

    // Create and run HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool across requests
            .configure(routes::configure)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
