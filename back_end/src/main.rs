use actix_web::{ web, App, HttpServer };

mod lib;
mod routers;
// Main function to configure and start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Blog posts
            .service(web::resource("/api/blog").route(web::get().to(routers::blog::get_blog_posts)))
            .service(
                web::resource("/api/blog/{id}").route(web::get().to(routers::blog::get_blog_post))
            )
            // GitHub stats
            .service(
                web
                    ::resource("/api/github-stats")
                    .route(web::get().to(routers::github_stats::get_github_stats))
            )
        // Add CORS middleware here if needed
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
