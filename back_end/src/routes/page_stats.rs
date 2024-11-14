use actix_web::{ get, web::Data, HttpRequest, HttpResponse };
use deadpool_postgres::Pool;
use serde_json::Value;

use crate::lib::{ schemas::page_stats::PageStats, types::error::ApiError };

#[get("/page-stats")]
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

async fn fetch_page_stats(pool: &Pool) -> Result<PageStats, ApiError> {
    let query =
        r#"
        WITH stats AS (
            SELECT
                COUNT(*) as page_views,
                COUNT(DISTINCT visitor_id) as unique_visitors,
                AVG(COALESCE(time_spent, 0)) as average_time_spent,
                (
                    COUNT(CASE WHEN bounce = true THEN 1 END)::float /
                    NULLIF(COUNT(DISTINCT session_id), 0)
                ) as bounce_rate
            FROM page_visits
            WHERE created_at >= NOW() - INTERVAL '24 hours'
        )
        SELECT 
            page_views,
            unique_visitors,
            ROUND(average_time_spent::numeric, 2) as average_time_spent,
            ROUND(COALESCE(bounce_rate, 0)::numeric * 100, 2) as bounce_rate
        FROM stats
    "#;

    // Get a connection from the pool
    let client = pool
        .get().await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

    // Execute the query
    let row = client
        .query_one(query, &[]).await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to execute query: {}", e)))?;

    // Convert the row to PageStats
    Ok(PageStats::from(row))
}
