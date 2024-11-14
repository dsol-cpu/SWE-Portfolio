use std::fmt;

use actix_web::HttpResponse;
use awc::http::StatusCode;

// Custom error type
#[derive(Debug)]
pub enum ApiError {
    DatabaseError(String),
    ConfigError(String),
    AuthError(String),
}

// Manual implementation of Error trait
impl std::error::Error for ApiError {}

// Manual implementation of Display trait
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ApiError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

// Implement conversion from ApiError to actix_web::Error
impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": self.to_string()
                })
                )
            }
            ApiError::ConfigError(_) => {
                HttpResponse::InternalServerError().json(
                    serde_json::json!({
                    "error": "Internal server configuration error"
                })
                )
            }
            ApiError::AuthError(_) => {
                HttpResponse::Unauthorized().json(
                    serde_json::json!({
                    "error": self.to_string()
                })
                )
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
