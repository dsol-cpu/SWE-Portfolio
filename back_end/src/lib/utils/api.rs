use std::{ env::VarError, fmt };

use actix_web::{ HttpResponse, ResponseError };
use awc::http::StatusCode;
use serde::Serialize;
use crate::lib::schemas::github_stats::GithubStats;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// Error handling
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized(String),
    RateLimited(String),
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not Found: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::RateLimited(msg) => write!(f, "Rate Limited: {}", msg),
            Self::Internal(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response: ApiResponse<GithubStats> = ApiResponse {
            success: false,
            data: None,
            error: Some(self.to_string()),
        };

        HttpResponse::build(self.status_code()).json(response)
    }
}

impl From<VarError> for ApiError {
    fn from(_: VarError) -> Self {
        ApiError::Internal("GitHub token not configured".to_string())
    }
}

impl From<awc::error::SendRequestError> for ApiError {
    fn from(error: awc::error::SendRequestError) -> Self {
        ApiError::Internal(format!("Request failed: {}", error))
    }
}

impl From<awc::error::PayloadError> for ApiError {
    fn from(error: awc::error::PayloadError) -> Self {
        ApiError::Internal(format!("Failed to read response: {}", error))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::Internal(format!("Failed to parse JSON: {}", error))
    }
}
