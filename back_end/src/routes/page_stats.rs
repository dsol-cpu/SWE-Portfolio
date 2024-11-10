use actix_web::{ error, get, http::StatusCode, HttpRequest, HttpResponse, Responder };
use serde_json::{ json, Error as JsonError };

#[get("/api/page-stats")]
pub async fn get_rest_api_page_stats(
    req: HttpRequest
) -> Result<impl Responder, actix_web::Error> {}

#[get("/graphql/page-stats")]
pub async fn get_graphql_page_stats(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {}
