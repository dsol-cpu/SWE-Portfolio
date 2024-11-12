use actix_web::{ error, get, http::StatusCode, HttpRequest, HttpResponse, Responder };
use serde_json::{ json, Error as JsonError };

#[get("/restapi/page-stats")]
pub async fn get_page_stats() -> impl Responder {}
