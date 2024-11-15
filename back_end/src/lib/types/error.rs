use std::fmt;
use actix_web::HttpResponse;
use awc::http::StatusCode;

// Custom error type
#[derive(Debug)]
pub enum ApiError {
    DatabaseError(String),
    ConfigError(String),
    AuthError(String),
    Internal(String),
    NotFound(String),
    Unauthorized(String),
    RateLimited(String),
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
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::RateLimited(msg) => write!(f, "Rate limited: {}", msg),
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
            ApiError::AuthError(_) | ApiError::Unauthorized(_) => {
                HttpResponse::Unauthorized().json(
                    serde_json::json!({
                        "error": self.to_string()
                    })
                )
            }
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(
                    serde_json::json!({
                        "error": msg
                    })
                )
            }
            ApiError::RateLimited(msg) => {
                HttpResponse::TooManyRequests().json(
                    serde_json::json!({
                        "error": msg
                    })
                )
            }
            ApiError::Internal(msg) => {
                HttpResponse::InternalServerError().json(
                    serde_json::json!({
                        "error": msg
                    })
                )
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthError(_) | ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Optional: Add conversions from other error types
impl From<std::env::VarError> for ApiError {
    fn from(err: std::env::VarError) -> Self {
        ApiError::ConfigError(err.to_string())
    }
}

impl From<awc::error::SendRequestError> for ApiError {
    fn from(err: awc::error::SendRequestError) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl From<awc::error::JsonPayloadError> for ApiError {
    fn from(err: awc::error::JsonPayloadError) -> Self {
        ApiError::Internal(format!("JSON parsing error: {}", err))
    }
}