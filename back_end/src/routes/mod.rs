use actix_web::web;

mod github_stats;
mod page_stats;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(page_stats::get_page_stats);
}
