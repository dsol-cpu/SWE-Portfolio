use actix_web::{ web, App, HttpServer };
use lib::utils::supabase::init_database;

mod lib;
mod routes;
mod constants;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Initialize the database connection pool
    let config = init_database().await.expect("Failed to initialize database pool");

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(config.clone())).configure(routes::configure)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
