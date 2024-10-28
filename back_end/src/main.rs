use actix_web::{ web, App, HttpServer };

mod lib;
mod routers;
// Main function to configure and start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Project routes
            .service(
                web
                    ::resource("/api/projects")
                    .route(web::get().to(routers::project_info::get_projects))
            )
            .service(
                web
                    ::resource("/api/projects/{id}")
                    .route(web::get().to(routers::project_info::get_project))
            )
            // Contact form
            .service(
                web
                    ::resource("/api/contact")
                    .route(web::post().to(routers::contact::submit_contact))
            )
            // Resume download
            .service(
                web::resource("/api/resume").route(web::get().to(routers::resume::download_resume))
            )
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
            // Weather data
            .service(
                web::resource("/api/weather").route(web::get().to(routers::weather::get_weather))
            )
        // Add CORS middleware here if needed
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
