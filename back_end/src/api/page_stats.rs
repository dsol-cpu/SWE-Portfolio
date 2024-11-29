use actix_web::{ get, web::Data, HttpRequest, HttpResponse };
use deadpool_postgres::Pool;

use crate::{ utils::page::fetch_page_stats, types::error::ApiError };

#[get("/api/page-stats")]
pub async fn get_page_stats(req: HttpRequest, data: Data<Pool>) -> Result<HttpResponse, ApiError> {
    // Validate authorization
    let _auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ApiError::AuthError("Missing authorization header".into()))?;

    // Fetch and return stats
    let stats = fetch_page_stats(&data).await?;
    Ok(HttpResponse::Ok().json(stats))
}
